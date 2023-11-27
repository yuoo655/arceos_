pub use starfive_eth::{StmmacDevice, StarfiveHal,mdio_write};

use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};
use crate::{EthernetAddress, NetBufPtr, NetDriverOps, net_buf};


unsafe impl<A: StarfiveHal> Sync for StarfiveNic<A> {}
unsafe impl<A: StarfiveHal> Send for StarfiveNic<A> {}


extern crate alloc;
use core::{fmt::write, marker::PhantomData};
use core::ptr::{read_volatile, write_volatile, self};

pub struct StarfiveNic<A>
where
    A: StarfiveHal,
{
    device: StmmacDevice<A>,
    phantom: PhantomData<A>,
}


impl<A: StarfiveHal> StarfiveNic<A> {
    pub fn init() -> Self {

        let ioaddr = A::phys_to_virt(0x1002_0000);

        let mut device = StmmacDevice::new();

        
        // phy set default reg space
        mdio_write::<A>(ioaddr,0xa001 ,0x783);
        
        // phy reset
        mdio_write::<A>(ioaddr,0x9000 ,0x3);

        // rgmii_clk_delay_config
        mdio_write::<A>(ioaddr,0x0 ,0x7c3);
        mdio_write::<A>(ioaddr,0xa003 ,0x783);
        mdio_write::<A>(ioaddr,0xa012 ,0x783);
        mdio_write::<A>(ioaddr,0xfd ,0x7c3);
        mdio_write::<A>(ioaddr,0xa001 ,0x783);

        // phy resume
        mdio_write::<A>(ioaddr,0x27 ,0x783);
        mdio_write::<A>(ioaddr,0xa00a ,0x783);

        device.dma_reset();
        device.dma_set_bus_mode();
        device.set_rxtx_base();
        device.set_mac_addr();
        device.dma_rxtx_enable();


        // phy starup
        mdio_write::<A>(ioaddr,0x27,0x783);
        mdio_write::<A>(ioaddr,0xa00a,0x783);
        // phy auto negotiation
        mdio_write::<A>(ioaddr,0x1de1,0x103);
        mdio_write::<A>(ioaddr,0x200,0x243);
        mdio_write::<A>(ioaddr,0x1200,0x3);

        device.stmmac_mac_link_up();

        Self {
            device: device,
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

        use core::ptr::NonNull;

        if let Some((skb_pa, len)) = self.device.receive(){
            let buffer_ptr = NonNull::new(skb_pa).expect("-------");
            let packet_ptr = NonNull::new(skb_pa).expect("-------");
            let net_buf = NetBufPtr::new(buffer_ptr, packet_ptr, len as usize);
            Ok(net_buf)
        }else{
            Err(DevError::Again)
        }
        
    }

    fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {

        let packet_va: *mut u8 = tx_buf.raw_ptr();
        let packet_pa = A::virt_to_phys(packet_va as usize);
        let len = tx_buf.len;

        self.device.transmit(packet_pa as usize, len);

        Err(DevError::Unsupported)
    }

    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> {
        Err(DevError::Unsupported)
    }
}
