use super::cvitek_defs::*;
use alloc::boxed::Box;
use alloc::slice;
use alloc::sync::Arc;
use alloc::{collections::VecDeque, vec::Vec};
use core::marker::PhantomData;
use core::ptr::{NonNull, read_volatile, write_volatile};
use core::time::Duration;
use core::{mem, ptr};

pub type IrqHandler = fn();

use super::cvitek_defs::*;
pub struct CvitekNicDevice<A: CvitekNicTraits> {
    iobase_pa: usize,
    iobase_va: usize,
    rx_rings: RxRing<A>,
    tx_rings: TxRing<A>,
    phantom: PhantomData<A>,
}

pub fn receive_irq_handler()
{
    info!("receive a package");
}

impl <A: CvitekNicTraits> CvitekNicDevice<A> {
    pub fn new(iobase_pa: usize) -> Self {
        let rx_ring = RxRing::<A>::new();
        let tx_ring = TxRing::<A>::new();
        let iobase_va = A::phys_to_virt(iobase_pa);
        let mut nic = CvitekNicDevice::<A> {
            iobase_pa,
            iobase_va,
            rx_rings: rx_ring,
            tx_rings: tx_ring,
            phantom: PhantomData,
        };
        A::register_irq(GMAC_IRQ,receive_irq_handler);
        nic.init();
        nic
    }

    pub fn init(&mut self) {
        // reset mac
        info!("try to reset dma!");
        let start_time:usize =A::current_time();
        unsafe{
            let mut val=read_volatile((self.iobase_va+GMAC_DMA_REG_BUS_MODE) as *mut u32);
            write_volatile((self.iobase_va+GMAC_DMA_REG_BUS_MODE) as *mut u32, val | DMAMAC_SRST);
            val=read_volatile((self.iobase_va+GMAC_DMA_REG_BUS_MODE) as *mut u32);
            while (val &DMAMAC_SRST)!=0{
                val=read_volatile((self.iobase_va+GMAC_DMA_REG_BUS_MODE) as *mut u32);
                if (A::current_time()-start_time)>=CONFIG_MDIO_TIMEOUT {
                    info!("DMA reset timeout\n");
                }
                A::mdelay(100);
            }
        }
        info!("finish try to reset dma!");
        // alloc rx_ring and tx_ring
        self.rx_rings.init_dma_desc_rings();
        self.tx_rings.init_dma_desc_rings();
        // set mac regs
        unsafe{
            write_volatile((self.iobase_va+GMAC_DMA_REG_OPMODE) as *mut u32, 0x2202906);
            write_volatile((self.iobase_va+GMAC_DMA_REG_BUS_MODE) as *mut u32, 0x3900800);
            write_volatile((self.iobase_va+GMAC_REG_CONF) as *mut u32, 0x41cc00);
            write_volatile((self.iobase_va+GMAC_DMA_REG_INTENABLE) as *mut u32, 0x10040);
        }
        
        info!("init tx and rxring\n");
    }
    pub fn read_mac_address(&self) -> [u8; 6]
    {
        let mut ret:[u8;6]=[0; 6];
        unsafe{
            let hi=read_volatile((self.iobase_va + GMAC_REG_MACADDR0HI) as *mut u32);
            let lo=read_volatile((self.iobase_va + GMAC_REG_MACADDR0LO) as *mut u32);
            ret[0]=(lo & 0xff) as u8;
            ret[1]=((lo>>8)& 0xff) as u8;
            ret[2]=((lo>>16)& 0xff) as u8;
            ret[3]=((lo>>24)& 0xff) as u8;
            ret[4]=(hi& 0xff) as u8;
            ret[5]=((hi>>8)& 0xff) as u8;
        }
        ret
    }


    pub fn get_tx_idx(&self) -> usize {
        let idx = self.tx_rings.idx;
        idx
    }

    pub fn receive(&mut self) -> Option<Packet> {
        let mut rx_rings = &mut self.rx_rings;
        let rd_dma = &mut rx_rings.rd;

        let mut status = 0;
        let mut idx = rx_rings.idx;
        let mut clean_idx = 0;

        let rd = rd_dma.read_volatile(idx).unwrap();
        let rdes0 = rd.txrx_status;
        let rdes1 = rd.dmamac_cntl;
        let rdes2 = rd.dmamac_addr;
        let rdes3 = rd.dmamac_cntl;

        status = rdes0 & (1 << 31);

        if status >> 31 == 1 {
            info!("dma own");
            return None;
        }

        // good frame
        // clean_idx = idx;
        let frame_len = rdes1 ;

        // get data from skb
        let skb_va = rx_rings.skbuf[idx] as *mut u8;
        let packet = Packet::new(skb_va, frame_len as usize);

        // alloc new skbuf
        // let (skb_new_va, skb_new_pa) = A::dma_alloc_pages(1);
        // rx_rings.set_idx_addr_owner(clean_idx, skb_new_pa);
        // rx_rings.skbuf[idx] = skb_new_va as _;

        rx_rings.idx = (idx + 1) % 512;
        return Some(packet);
    }

    pub fn transmit(&mut self, packet: Packet) {
        let packet_va = packet.skb_va as usize;
        let packet_pa = A::virt_to_phys(packet_va);
        let packet_len = packet.len as usize;
        let tx_rings: &mut TxRing<A> = &mut self.tx_rings;
        let idx: usize = tx_rings.idx;

        tx_rings.set_idx_addr_owner(idx, true, true, false, true, packet_pa, packet_len);

        tx_rings.idx = (idx + 1) % 512;

        tx_rings.set_tail_ptr(self.iobase_va);
    }

}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Des {
    pub txrx_status: u32,
    pub dmamac_cntl: u32,
    pub dmamac_addr: u32,
    pub dmamac_next: u32,
}

pub struct RxRing<A: CvitekNicTraits> {
    pub idx: usize,
    pub skbuf: Vec<usize>,
    pub rd: Dma<Des>,
    phantom: PhantomData<A>,
}

impl<A:CvitekNicTraits> RxRing<A> {
    pub fn new() -> Self {
        let count = CONFIG_RX_DESCR_NUM;
        let size = mem::size_of::<Des>() * count;
        let pa = 0x89000000 as usize;
        let va = A::phys_to_virt(pa);

        info!("rx des  pa: {:#x?} end {:#x?}", pa, pa + size);
        let rd_dma = Dma::new(va as _, pa, count);
        let skbuf: Vec<usize> = Vec::new();
        Self {
            rd: rd_dma,
            phantom: PhantomData,
            idx: 0,
            skbuf: skbuf,
        }
    }

    pub fn init_dma_desc_rings(&mut self) {
        info!("rx init dma_desc_rings");
        
    }

    /// Release the next RDes to the DMA engine
    pub fn set_idx_addr_owner(&mut self, idx: usize, skb_phys_addr: usize) {
        let mut rd = Des {
            txrx_status: 0,
            dmamac_cntl: 0,
            dmamac_addr: 0,
            dmamac_next: 0,
        };

        // dwmac_desc_set_addr
        rd.txrx_status = skb_phys_addr as u32;
        rd.dmamac_cntl = ((skb_phys_addr >> 32) & 0xFF) as u32;

        // dwmac_set_rx_owner
        // rd.rdes3 |= RDES3_INT_ON_COMPLETION_EN;

        self.rd.write_volatile(idx, &rd);

        /*unsafe {
            core::arch::asm!("dsb sy");
        }*/
    }

    pub fn set_head_tail_ptr(&mut self, iobase: usize) {
        
        let rd_addr = self.rd.phy_addr as usize;

    }
}

pub struct TxRing<A: CvitekNicTraits> {
    pub idx: usize,
    pub skbuf: Vec<usize>,
    pub td: Dma<Des>,
    phantom: PhantomData<A>,
}

impl<A: CvitekNicTraits> TxRing<A> {
    pub fn new() -> Self {
        let count = 512;

        let size = mem::size_of::<Des>() * count;
        let pa = 0x89000000 + 0x3000 as usize;
        let va = A::phys_to_virt(pa);

        info!("tx des  pa: {:#x?} end {:#x?}", pa, pa + size);
        let td_dma: Dma<Des> = Dma::new(va as _, pa, count);
        
        let skbuf = Vec::new();
        Self {
            td: td_dma,
            phantom: PhantomData,
            idx: 0,
            skbuf: skbuf,
        }
    }
    pub fn init_dma_desc_rings(&mut self) {
        info!("tx set_idx_addr_owner");
    }

    pub fn set_idx_addr_owner(
        &mut self,
        idx: usize,
        fs: bool,
        ls: bool,
        csum: bool,
        own: bool,
        skb_phys_addr: usize,
        len: usize,
    ) {
        let skb_va = A::phys_to_virt(skb_phys_addr);
        self.skbuf.push(skb_va);
        let td = self.td.read_volatile(idx).unwrap();

        let mut td = Des {
            txrx_status: 0,
            dmamac_cntl: 0,
            dmamac_addr: 0,
            dmamac_next: 0,
        };

        td.txrx_status = skb_phys_addr as u32; // Buffer 1
        td.dmamac_cntl = ((skb_phys_addr >> 32) & 0xff) as u32; // Not used


        self.td.write_volatile(idx, &td);
    }

    pub fn set_tail_ptr(&mut self, iobase: usize) {
        let td_addr = self.td.phy_addr as usize;
        let idx = self.idx;
        info!("tx set_tail_ptr idx:{:?}", idx);
    }
}

pub struct Dma<T> {
    pub count: usize,
    pub phy_addr: usize,
    pub cpu_addr: *mut T,
}

impl<T> Dma<T> {
    pub fn new(cpu_addr: *mut T, phy_addr: usize, count: usize) -> Self {
        Self {
            count: count,
            phy_addr: phy_addr,
            cpu_addr: cpu_addr,
        }
    }

    pub fn read_volatile(&self, index: usize) -> Option<T> {
        if index >= self.count {
            // pr_info!("read_volatile index:{:?} count:{:?}", index, self.count);
            return None;
        }

        let ptr = self.cpu_addr.wrapping_add(index);

        // SAFETY: We just checked that the index is within bounds.
        Some(unsafe { ptr.read() })
    }

    pub fn write_volatile(&self, index: usize, value: &T) -> bool
    where
        T: Copy,
    {
        if index >= self.count {
            // pr_info!("read_volatile index:{:?} count:{:?}", index, self.count);
            return false;
        }

        let ptr = self.cpu_addr.wrapping_add(index);

        // SAFETY: We just checked that the index is within bounds.
        unsafe { ptr.write(*value) };
        true
    }
}

pub struct Packet {
    pub skb_va: *mut u8,
    pub len: usize,
}

impl Packet {
    pub fn new(skb_va: *mut u8, len: usize) -> Self {
        Self {
            skb_va: skb_va,
            len: len,
        }
    }

    /// Returns all data in the buffer, not including header.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.skb_va, self.len) }
    }

    /// Returns all data in the buffer with the mutuable reference,
    /// not including header.
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.skb_va, self.len) }
    }
}

pub trait CvitekNicTraits {
    fn phys_to_virt(pa: usize) -> usize {
        pa
    }
    fn virt_to_phys(va: usize) -> usize {
        va
    }
    fn dma_alloc_pages(pages: usize) -> (usize, usize);

    fn dma_free_pages(vaddr: usize, pages: usize);

    fn mdelay(m_times: usize);

    fn current_time() -> usize;

    fn register_irq(irq_num: usize, handler: IrqHandler) -> bool;
}