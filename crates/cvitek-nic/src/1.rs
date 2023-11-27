use core::{fmt::write, marker::PhantomData};

use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};

use crate::{EthernetAddress, NetBufPtr, NetDriverOps};

extern crate alloc;

unsafe impl<A: StarfiveHal> Sync for StarfiveNic<A> {}
unsafe impl<A: StarfiveHal> Send for StarfiveNic<A> {}

pub const DMA_BUS_MODE: usize = 0x00001000;

/* SW Reset */
pub const DMA_BUS_MODE_SFT_RESET: usize = 0x00000001; /* Software Reset */

/* AXI Master Bus Mode */
pub const DMA_AXI_BUS_MODE: usize = 0x00001028;

pub const DMA_RCV_BASE_ADDR: usize = 0x0000100c; /* Receive List Base */

/* Ctrl (Operational Mode) */
pub const DMA_CONTROL: usize = 0x00001018;

pub const DMA_CONTROL_SR: usize = 0x00000002;

pub const MAC_ENABLE_TX: u32 = 1 << 3; /* Transmitter Enable */
pub const MAC_ENABLE_RX: u32 = 1 << 2; /* Receiver Enable */

/* Received Poll Demand */
pub const DMA_XMT_POLL_DEMAND: u32 = 0x00001004;

/* Received Poll Demand */
pub const DMA_RCV_POLL_DEMAND: u32 = 0x00001008;


pub const DMA_CONTROL_ST:u32 = 		0x00002000	;

pub const SIFIVE_CCACHE_WAY_ENABLE:usize = 0x8;


use core::ptr::{read_volatile, write_volatile};

pub struct StarfiveNic<A>
where
    A: StarfiveHal,
{
    ioaddr: usize,
    phantom: PhantomData<A>,
}


pub fn sifive_ccache_flush_range<A: StarfiveHal>(start: usize, end:usize){


    // let start_pa = A::virt_to_phys(start) as u32;
    // let end_pa: u32 = A::virt_to_phys(end) as u32;
    log::info!("sifive_ccache_flush_range---------start:{:#x} end:{:#x?}", start, end);
    let start_pa = start as usize;
    let end_pa = end as usize;

    let mut s = start_pa;

    let cache_line_size = 0x40;

    let cache_flush = A::phys_to_virt(0x201_0000);

    A::fence();

    unsafe{
        core::arch::asm!("fence")
    };

    let addr = cache_flush + 0x200 as usize;

    // let va = A::phys_to_virt(addr);



    // let ptr = &va as _ as usize;
    // let ptr = &va as *const usize as usize;




    while s < end_pa as usize{


        // let flush64 = *((cache_flush + 0x200) as *mut u32);
        unsafe{
            write_volatile((cache_flush + 0x200) as *mut usize, s);
        }
        unsafe{
            write_volatile((cache_flush + 0x200) as *mut usize, A::phys_to_virt(s));
        }

        s += cache_line_size;
    }
    A::fence();

    unsafe{
        core::arch::asm!("fence")
    };
}


impl<A: StarfiveHal> StarfiveNic<A> {
    pub fn init() -> Self {

        let cache_flush = A::phys_to_virt(0x201_0000);

        log::info!("-----------cache_flush_enable-----------");
        unsafe{
            write_volatile((cache_flush + SIFIVE_CCACHE_WAY_ENABLE) as *mut u32, 0xf)
        }

        let cache_line_size = 0x40;
        

        sifive_ccache_flush_range::<A>(0x1800_1000 as usize, 0x1800_2000);
        sifive_ccache_flush_range::<A>(0x1801_0000 as usize, 0x1801_f000 as usize);



        let ioaddr = A::phys_to_virt(0x1002_0000);

        dump_reg(ioaddr);





        log::info!("init_dma_rx_desc_rings");

        let mut rx_ring = RxRing::<A>::new();
        A::fence();
        let rdes_base = rx_ring.rd.phy_addr as u32;

        let skb_start = 0x1802_0000 as usize;
        for i in 0..16 {
            let buff_addr = skb_start + 0x1000 * i;
            rx_ring.init_rx_desc(i, buff_addr);
        }
        dump_reg(ioaddr);



        log::info!("init_dma_tx_desc_rings");
        let mut tx_ring = TxRing::<A>::new();
        A::fence();
        let tdes_base = tx_ring.td.phy_addr as u32;
        let tskb_start = 0x1801_0000 as usize;
        for i in 0..16 {
            tx_ring.init_tx_desc(i,  false);
        }
        tx_ring.init_tx_desc(15,  true);
        

        sifive_ccache_flush_range::<A>(0x1800_1000 as usize, 0x1800_2000);
        sifive_ccache_flush_range::<A>(0x1801_0000 as usize, 0x1801_f000 as usize);


        mdio_write::<A>(ioaddr,0xa001 ,0x783);

        mdio_write::<A>(ioaddr,0x9000 ,0x3);
        mdio_write::<A>(ioaddr,0x0 ,0x7c3);
        mdio_write::<A>(ioaddr,0xa012 ,0x783);

        mdio_write::<A>(ioaddr,0x9000 ,0x3);
        mdio_write::<A>(ioaddr,0xa001 ,0x783);
        mdio_write::<A>(ioaddr,0xa003 ,0x783);
        mdio_write::<A>(ioaddr,0xfd ,0x7c3);
        mdio_write::<A>(ioaddr,0xa001 ,0x783);

        mdio_write::<A>(ioaddr,0x27 ,0x783);
        mdio_write::<A>(ioaddr,0xa00a ,0x783);





        A::fence();
        dump_reg(ioaddr);




        unsafe{
            log::info!("-------------dwmac_dma_reset--------------------");
            let mut value = read_volatile((ioaddr + DMA_BUS_MODE) as *mut u32);
        
            value |= DMA_BUS_MODE_SFT_RESET as u32;

            write_volatile((ioaddr + DMA_BUS_MODE) as *mut u32, value);
            A::mdelay(100);
            loop{
                let value = read_volatile((ioaddr + DMA_BUS_MODE) as *mut u32);
                if value != value & DMA_BUS_MODE_SFT_RESET as u32{
                    break;
                }
            }
        }
        log::info!("-------------dwmac_dma_reset done--------------------");

        dump_reg(ioaddr);



        log::info!("---------------set dma bus mode------------------------------");
        unsafe{
            write_volatile((ioaddr + DMA_BUS_MODE) as *mut u32, 0x910880);
            // write_volatile((ioaddr + 0x1028) as *mut u32, 0xf0);
        }

        log::info!("-------------set tx base --------------------");
        unsafe {
            write_volatile((ioaddr + 0x1010) as *mut u32, tdes_base);
        }
                
        log::info!("-------------set rx base --------------------");
        unsafe {

            write_volatile((ioaddr + DMA_RCV_BASE_ADDR) as *mut u32, rdes_base);
        }





        log::info!("set mac addr");
        let mac_id: [u8; 6] = [0xaa, 0xbb, 0xcc, 0xdd, 0x05, 0x06];

        let macid_lo = 0xddccbbaa;

        let macid_hi = 0x0605;

        unsafe {
            write_volatile((ioaddr + 0x40) as *mut u32, macid_hi);
        }

        unsafe {
            write_volatile((ioaddr + 0x44) as *mut u32, macid_lo);
        }




        log::info!("------------------enable mac core init ------------------");
        unsafe{
            write_volatile((ioaddr) as *mut u32, 0x618000);
        }





        log::info!("--------------------enable mac rx/tx-----------------------");
        stmmac_set_mac(ioaddr, true);

        
        log::info!("-----------------------------stmmac_dma_operation_mode---------------------");
        unsafe{
            write_volatile((ioaddr + DMA_CONTROL) as *mut u32, 0x900);
            write_volatile((ioaddr + DMA_CONTROL) as *mut u32, 0x0);
        }


        log::info!("---------start dma tx/rx----------------------------");
        unsafe {
            let mut value = read_volatile((ioaddr + DMA_CONTROL) as *mut u32);
            value |= 0x2;
            write_volatile((ioaddr + DMA_CONTROL) as *mut u32, value);


            let mut value = read_volatile((ioaddr + DMA_CONTROL) as *mut u32);
            value |= DMA_CONTROL_ST;
            write_volatile((ioaddr + DMA_CONTROL) as *mut u32, value);
            
        }

        dump_reg(ioaddr);



        log::info!("------------------------------------phy_starup--------------------");

        mdio_write::<A>(ioaddr,0x27,0x783);
        mdio_write::<A>(ioaddr,0xa00a,0x783);

        mdio_write::<A>(ioaddr,0x1de1,0x103);
        mdio_write::<A>(ioaddr,0x200,0x243);
        mdio_write::<A>(ioaddr,0x1200,0x3);
        unsafe{
            write_volatile((ioaddr + 0x18) as *mut u32, 0xe);
        }


        unsafe{
            write_volatile((ioaddr) as *mut u32, 0x61080c);
        }
        






        for i in 0..16{
            let td = tx_ring.td.read_volatile(i).unwrap();
            log::info!("td {:x?}", td);
        }

        log::info!("-------------------sending------------------------");
        let x: &mut [u8] = &mut [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xaa, 0xbb, 0xcc, 0xdd, 0x05, 0x06, 0x08, 0x06, 0x00,0x01, 
            0x08, 0x00, 0x06, 0x04, 0x00, 0x01, 0xaa, 0xbb, 0xcc, 0xdd, 0x05, 0x06, 0xc0, 0xa8,
            0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0xa8, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        
        





        
            for i in 0..16{
            
                let buff_addr = tskb_start + 0x1000 * i;
                let raw_pointer = x.as_mut_ptr();
                let packet_pa: usize = tskb_start + 0x1000 * i;
                let packet_va = A::phys_to_virt(packet_pa);
                let buff = packet_va as *mut u8;
                unsafe {
                    core::ptr::copy_nonoverlapping(raw_pointer as *const u8, buff as *mut u8, 0x2a);
                }
    
                let mut td = tx_ring.td.read_volatile(i).unwrap();
                
                td.tdes2 = buff_addr as u32;
                td.tdes1 = 0x3c;
                // td.tdes0 &= !(DESC_TXSTS_MSK);
                td.tdes0 |= 1 << 29;
                td.tdes0 |= 1 << 28;
                td.tdes0 |= 1 << 31;

                tx_ring.td.write_volatile(i, &td);
                unsafe{
                    core::arch::asm!("fence	ow,ow");
                }
                A::fence();

            
                sifive_ccache_flush_range::<A>(0x1800_1000 as usize, 0x1800_2000);
                sifive_ccache_flush_range::<A>(0x1801_0000 as usize, 0x1801_f000 as usize);

                A::mdelay(500);
                log::info!("td {:x?}", td);
                
                unsafe{
                    write_volatile((ioaddr + 0x1004) as *mut u32, 0x1);
                }


                // let value = unsafe{
                    //     read_volatile((ioaddr + 0x104c) as *mut u32)
                    // };
                    // log::info!("Current Host rx descriptor -----{:#x?}", value);
                A::mdelay(1000);    

                    
                let value = unsafe{
                    read_volatile((ioaddr + 0x1048) as *mut u32)
                };
                log::info!("Current Host tx descriptor -----{:#x?}", value);

                
                let intr_status = unsafe{
                    read_volatile((ioaddr + 0x1014) as *mut u32)
                };
                let state = (intr_status & 0x00700000) >> 20;
                log::info!("tx state{:?}", state);
                
            }
            


        // for i in 0..10 {
        //     A::mdelay(2000);

        //     for i in 0..5 {
        //         let rd = rx_ring.rd.read_volatile(i).unwrap();
        //         log::info!("rd {:x?}", rd);
        //     }
        //     let value = unsafe{
        //         read_volatile((ioaddr + 0x104c) as *mut u32)
        //     };
        //     log::info!("Current Host rx descriptor -----{:#x?}", value);
        
        // }



        Self {
            ioaddr: 0x10020000,
            phantom: PhantomData,
        }
    }
}

impl<A: StarfiveHal> BaseDriverOps for StarfiveNic<A> {
    fn device_name(&self) -> &str {
        "starfive"
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Net
    }
}

impl<A: StarfiveHal> NetDriverOps for StarfiveNic<A> {
    fn mac_address(&self) -> crate::EthernetAddress {
        crate::EthernetAddress([0xaa, 0xbb, 0xcc, 0xdd, 0x05, 0x06])
    }

    fn tx_queue_size(&self) -> usize {
        1
    }

    fn rx_queue_size(&self) -> usize {
        1
    }

    fn can_receive(&self) -> bool {
        true
    }

    fn can_transmit(&self) -> bool {
        true
    }

    fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult {
        Err(DevError::Unsupported)
    }

    fn recycle_tx_buffers(&mut self) -> DevResult {
        Err(DevError::Unsupported)
    }

    fn receive(&mut self) -> DevResult<NetBufPtr> {
        Err(DevError::Unsupported)
    }

    fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {
        Err(DevError::Unsupported)
    }

    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> {
        Err(DevError::Unsupported)
    }
}

pub const MII_BUSY: u32 = (1 << 0);
pub const MII_WRITE: u32 = (1 << 1);
pub const MII_CLKRANGE_60_100M: u32 = (0);
pub const MII_CLKRANGE_100_150M: u32 = (0x4);
pub const MII_CLKRANGE_20_35M: u32 = (0x8);
pub const MII_CLKRANGE_35_60M: u32 = (0xC);
pub const MII_CLKRANGE_150_250M: u32 = (0x10);
pub const MII_CLKRANGE_250_300M: u32 = (0x14);
pub const MIIADDRSHIFT: u32 = (11);
pub const MIIREGSHIFT: u32 = (6);
pub const MII_REGMSK: u32 = (0x1F << 6);
pub const MII_ADDRMSK: u32 = (0x1F << 11);



use alloc::vec::Vec;
use core::mem;

#[derive(Debug)]
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
        Some(unsafe { ptr.read() })
    }

    pub fn write_volatile(&self, index: usize, value: &T) -> bool
    where
        T: Copy,
    {
        if index >= self.count {
            return false;
        }
        let ptr = self.cpu_addr.wrapping_add(index);
        unsafe { ptr.write(*value) };
        true
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct RxDes {
    pub rdes0: u32,
    pub rdes1: u32,
    pub rdes2: u32,
    pub rdes3: u32,
}

pub struct RxRing<A> {
    pub rd: Dma<RxDes>,
    pub idx: usize,
    pub skbuf: Vec<usize>,
    phantom: PhantomData<A>,
}

impl<A: StarfiveHal> RxRing<A> {
    pub fn new() -> Self {
        let count = 64;
        // let size = mem::size_of::<RxDes>() * count;
        // let pages = (size + 0x1000 - 1) / 0x1000;
        // let (va, pa) = A::dma_alloc_pages(pages);

        let pa = 0x1800_2000;
        let va = A::phys_to_virt(pa);

        let rd_dma = Dma::new(va as _, pa, count);
        let skbuf = Vec::new();

        Self {
            rd: rd_dma,
            idx: 0,
            skbuf: skbuf,
            phantom: PhantomData,
        }
    }

    pub fn init_rx_desc(&mut self, idx: usize, skb_phys_addr: usize) {
        let mut rd = RxDes {
            rdes0: 0,
            rdes1: 0,
            rdes2: 0,
            rdes3: 0,
        };
        //RDES0_OWN
        rd.rdes0 = 1 << 31 as u32;
        //buff size
        rd.rdes1 |= 0x600;

        rd.rdes2 = skb_phys_addr as u32;

        self.rd.write_volatile(idx, &rd);
    }
}


#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct TxDes {
    pub tdes0: u32,
    pub tdes1: u32,
    pub tdes2: u32,
    pub tdes3: u32,

    pub tdes4: u32,
    pub tdes5: u32,
    pub tdes6: u32,
    pub tdes7: u32,
}

pub struct TxRing<A> {
    pub td: Dma<TxDes>,
    pub idx: usize,
    pub skbuf: Vec<usize>,
    phantom: PhantomData<A>,
}

impl<A: StarfiveHal> TxRing<A> {
    pub fn new() -> Self {
        let count = 16;
        let pa = 0x1800_1000;
        let va = A::phys_to_virt(pa);

        let td_dma = Dma::new(va as _, pa, count);
        let skbuf = Vec::new();

        Self {
            td: td_dma,
            idx: 0,
            skbuf: skbuf,
            phantom: PhantomData,
        }
    }

    pub fn init_tx_desc(&mut self, idx: usize, end:bool) {
        let mut td: TxDes = TxDes {
            tdes0: 0,
            tdes1: 0,
            tdes2: 0,
            tdes3: 0,

            tdes4: 0,
            tdes5: 0,
            tdes6: 0,
            tdes7: 0,
        };

        td.tdes0 &= !(1 << 31);


        if end {
            td.tdes0 |= 1 << 21;
        }
    
        self.td.write_volatile(idx, &td);
    }

    pub fn set_skb_addr(&mut self, idx: usize, skb_addr:usize){

        let mut td = self.td.read_volatile(idx).unwrap();
        td.tdes2 = skb_addr as u32;
        self.td.write_volatile(idx, &td);
    }
}


pub fn stmmac_set_mac(ioaddr: usize, enable: bool) {
    let old_val: u32;
    let mut value: u32;

    log::info!("stmmac_set_mac--------------------enable={:?}", enable);

    old_val = unsafe { read_volatile(ioaddr as *mut u32) };
    value = old_val;

    if enable {
        value |= MAC_ENABLE_RX | MAC_ENABLE_TX;
    } else {
        value &= !(MAC_ENABLE_TX | MAC_ENABLE_RX);
    }

    if value != old_val {
        unsafe { write_volatile(ioaddr as *mut u32, value) }
    }
}

pub trait StarfiveHal {
    fn phys_to_virt(pa: usize) -> usize {
        pa
    }
    fn virt_to_phys(va: usize) -> usize {
        va
    }

    fn dma_alloc_pages(pages: usize) -> (usize, usize);

    fn dma_free_pages(vaddr: usize, pages: usize);

    fn mdelay(m_times: usize);

    fn fence();
}

pub fn dump_reg(ioaddr: usize) {
    log::info!("------------------------------dumpreg--------------------------------------");
    for i in 0..23 {
        let value = unsafe { read_volatile((ioaddr + 0x00001000 + i * 4) as *mut u32) };
        log::info!("reg {:?} = {:#x?}", i, value);
    }
}



pub fn ytphy_read_ext<A: StarfiveHal>(iobase: usize, reg: u32) -> u32 {

    dw_mdio_write::<A>(iobase, 0x1e, reg);

    let value = dw_mdio_read::<A>(iobase, 0x1f);

    value

}

pub fn ytphy_write_ext<A: StarfiveHal>(iobase: usize, reg: u32, value: u32) {
    
    dw_mdio_write::<A>(iobase, 0x1e, reg);

    dw_mdio_write::<A>(iobase, 0x1f, value);

}

pub fn dw_mdio_write<A: StarfiveHal>(iobase: usize, reg: u32, value: u32) {

    let addr = 0x3;

    unsafe {
        write_volatile((iobase + 0x14) as *mut u32, value);
    }

    let mut miiaddr =
        ((addr << MIIADDRSHIFT) & MII_ADDRMSK) | ((reg << MIIREGSHIFT) & MII_REGMSK) | MII_WRITE;

    miiaddr = miiaddr | MII_CLKRANGE_150_250M | MII_BUSY;
    log::info!(
        "dw_mdio_write  addr={:#x?} reg={:#x?} val_0x14={:#x?}, val_0x10={:#x?}\n",
        addr,
        reg,
        value,
        miiaddr | MII_CLKRANGE_150_250M | MII_BUSY
    );

    unsafe {
        write_volatile((iobase + 0x10) as *mut u32, miiaddr);
    }

    loop {
        let value = unsafe { read_volatile((iobase + 0x10) as *mut u32) };

        if value & MII_BUSY != 1 {
            break;
        }
        A::mdelay(10);
    }
}

pub fn dw_mdio_read<A: StarfiveHal>(iobase: usize, reg: u32) -> u32 {

    let addr = 0x3;

    let mut miiaddr = ((addr << MIIADDRSHIFT) & MII_ADDRMSK) | ((reg << MIIREGSHIFT) & MII_REGMSK);

    miiaddr = miiaddr | MII_CLKRANGE_150_250M | MII_BUSY;


    log::info!("dw_mdio_read  reg={:#x?}", reg);

    unsafe {
        write_volatile((iobase + 0x10) as *mut u32, miiaddr);
    }

    loop {
        let value = unsafe { read_volatile((iobase + 0x10) as *mut u32) };

            
        if value & MII_BUSY != 1 {
            let value = unsafe { read_volatile((iobase + 0x14) as *mut u32) };
            return value;
        }
        A::mdelay(10);
    }
}



pub const DESC_TXSTS_OWNBYDMA		:u32 = (1 << 31);
pub const DESC_TXSTS_TXINT		:u32 = (1 << 30);
pub const DESC_TXSTS_TXLAST		:u32 = (1 << 29);
pub const DESC_TXSTS_TXFIRST		:u32 = (1 << 28);
pub const DESC_TXSTS_TXCRCDIS		:u32 = (1 << 27);


pub const DESC_TXSTS_TXPADDIS		    :u32 = (1 << 26);
pub const DESC_TXSTS_TXCHECKINSCTRL	:u32 = (3 << 22);
pub const DESC_TXSTS_TXRINGEND		:u32 = (1 << 21);
pub const DESC_TXSTS_TXCHAIN		    :u32 = (1 << 20);
pub const DESC_TXSTS_MSK			    :u32 = (0x1FFFF << 0);


pub const DESC_TXCTRL_SIZE1MASK	:u32 = 	(0x7FF << 0);
pub const DESC_TXCTRL_SIZE1SHFT	:u32 = 	(0);
pub const DESC_TXCTRL_SIZE2MASK	:u32 = 	(0x7FF << 11);
pub const DESC_TXCTRL_SIZE2SHFT	:u32 = 	(11);



pub fn mdio_write<A: StarfiveHal>(ioaddr: usize, data: u32, value: u32) {

    loop {
        let value = unsafe { read_volatile((ioaddr + 0x10) as *mut u32) };

        if value & MII_BUSY != 1 {
            break;
        }
        A::mdelay(10);
    }



    unsafe{
        write_volatile((ioaddr + 0x14) as *mut u32, data);
        write_volatile((ioaddr + 0x10) as *mut u32, value);
    }

    loop {
        let value = unsafe { read_volatile((ioaddr + 0x10) as *mut u32) };

        if value & MII_BUSY != 1 {
            break;
        }
        A::mdelay(10);
    }
}