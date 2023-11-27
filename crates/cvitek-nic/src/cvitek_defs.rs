/// cvitek mac definations

pub (crate) const GMAC_REG_BASE_ADDR: usize = 0x04070000;
pub (crate) const GMAC_REG_CONF:usize = 0x0;
pub (crate) const GMAC_REG_FRAMEFILT:usize = 0x04;
pub (crate) const GMAC_REG_HASHTABLEHIGH:usize = 0x08;
pub (crate) const GMAC_REG_HASHTABLELOW:usize = 0x0c;
pub (crate) const GMAC_REG_MIIADDR:usize = 0x10;
pub (crate) const GMAC_REG_MIIDATA:usize = 0x14;
pub (crate) const GMAC_REG_FLOWCONTROL:usize = 0x18;
pub (crate) const GMAC_REG_VLANTAG:usize = 0x1c;
pub (crate) const GMAC_REG_VERSION:usize = 0x20;
pub (crate) const GMAC_REG_INTREG:usize = 0x38;
pub (crate) const GMAC_REG_INTMASK:usize = 0x3c;
pub (crate) const GMAC_REG_MACADDR0HI:usize = 0x40;
pub (crate) const GMAC_REG_MACADDR0LO:usize = 0x44;


pub (crate) const GMAC_DMA_REG_BASE_ADDR: usize = 0x1000;
pub (crate) const GMAC_DMA_REG_BUS_MODE: usize = GMAC_DMA_REG_BASE_ADDR + 0x0;
pub (crate) const GMAC_DMA_REG_TXPOLLDEMAND: usize = GMAC_DMA_REG_BASE_ADDR + 0x04;
pub (crate) const GMAC_DMA_REG_RXPOLLDEMAND: usize = GMAC_DMA_REG_BASE_ADDR + 0x08;
pub (crate) const GMAC_DMA_REG_RXDESCLISTADDR: usize = GMAC_DMA_REG_BASE_ADDR + 0x0c;
pub (crate) const GMAC_DMA_REG_TXDESCLISTADDR: usize = GMAC_DMA_REG_BASE_ADDR + 0x10;
pub (crate) const GMAC_DMA_REG_STATUS: usize = GMAC_DMA_REG_BASE_ADDR + 0x14;
pub (crate) const GMAC_DMA_REG_OPMODE: usize = GMAC_DMA_REG_BASE_ADDR + 0x18;
pub (crate) const GMAC_DMA_REG_INTENABLE: usize = GMAC_DMA_REG_BASE_ADDR + 0x1c;
pub (crate) const GMAC_DMA_REG_DISCARDEDCOUNT: usize = GMAC_DMA_REG_BASE_ADDR + 0x20;
pub (crate) const GMAC_DMA_REG_WDTFORRI: usize = GMAC_DMA_REG_BASE_ADDR + 0x24;
pub (crate) const GMAC_DMA_REG_AXIBUS: usize = GMAC_DMA_REG_BASE_ADDR + 0x28;
pub (crate) const GMAC_DMA_REG_CURRHOSTTXDESC: usize = GMAC_DMA_REG_BASE_ADDR + 0x48;
pub (crate) const GMAC_DMA_REG_CURRHOSTRXDESC: usize = GMAC_DMA_REG_BASE_ADDR + 0x4c;
pub (crate) const GMAC_DMA_REG_CURRHOSTTXBUFFADDR: usize = GMAC_DMA_REG_BASE_ADDR + 0x50;
pub (crate) const GMAC_DMA_REG_CURRHOSTRXBUFFADDR: usize = GMAC_DMA_REG_BASE_ADDR + 0x5c;

//

pub (crate) const DMAMAC_SRST:u32= 1<<0;
pub (crate) const CONFIG_MDIO_TIMEOUT :usize = 3*1000;

pub (crate) const FIXEDBURST:u32=1<<16;
pub (crate) const PRIORXTX_41:u32= 3<<14;
pub (crate) const DMA_PBL:u32=8<<8;

pub (crate) const FLUSHTXFIFO:u32=1<<20;
pub (crate) const STOREFORWARD:u32=1<<21;

pub (crate) const RXSTART:u32=1<<1;
pub (crate) const TXSTART:u32=1<<13;

pub (crate) const CONFIG_TX_DESCR_NUM:usize =64;
pub (crate) const CONFIG_RX_DESCR_NUM:usize =64;

//irq

pub (crate) const GMAC_IRQ:usize = 31;



pub const DMA_BUS_MODE: usize = 0x00001000;

/* SW Reset */
pub const DMA_BUS_MODE_SFT_RESET: usize = 0x00000001; /* Software Reset */

/* AXI Master Bus Mode */
pub const DMA_AXI_BUS_MODE: usize = 0x00001028;


pub const DMA_RCV_BASE_ADDR: usize = 0x0000100c; /* Receive List Base */

pub const DMA_TX_BASE_ADDR: usize = 0x00001010; /* Receive List Base */

/* Ctrl (Operational Mode) */
pub const DMA_CONTROL: usize = 0x00001018;

pub const DMA_CONTROL_SR: u32 = 0x00000002;

pub const MAC_ENABLE_TX: u32 = 1 << 3; /* Transmitter Enable */
pub const MAC_ENABLE_RX: u32 = 1 << 2; /* Receiver Enable */

/* Received Poll Demand */
pub const DMA_XMT_POLL_DEMAND: usize = 0x00001004;

/* Received Poll Demand */
pub const DMA_RCV_POLL_DEMAND: u32 = 0x00001008;



pub const DMA_CONTROL_ST:u32 = 		0x00002000;


pub const SIFIVE_CCACHE_WAY_ENABLE:usize = 0x8;

pub const MAC_ADDR_HI: usize = 0x40;
pub const MAC_ADDR_LO: usize = 0x44;

pub const DESC_RXSTS_FRMLENMSK	:usize = 	0x3FFF << 16;
pub const DESC_RXSTS_FRMLENSHFT:usize = 		16;