pub (crate) const EPHY_EFUSE_VALID_BIT_BASE:u32 = 0x03050120;
pub (crate) const EPHY_EFUSE_TXITUNE_FLAG: u32 = 0x00000200;
pub (crate) const EPHY_EFUSE_EXECHORC_FLAG: u32 = 0x00000100;
pub (crate) const EPHY_EFUSE_TXRXTERM_FLAG: u32 = 0x00000800;

pub (crate) const MIIADDRSHIFT:u32 = 11;
pub (crate) const MIIADDRMASK:u32 = 0x1F << 11;
pub (crate) const MIIREGSHIFT:u32 = 6;
pub (crate) const MIIREGMASK:u32 = 0x1F << 6;

pub (crate) const MII_CLKRANGE_150_250M :u32 = 0x10;
pub (crate) const MII_BUSY: u32 = 1 << 0;

pub (crate) const MII_WRITE: u32 = 1 << 1;
pub (crate) const CONFIG_MDIO_TIMEOUT :usize = 3*1000;

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

pub (crate) const CVI_MII_PHYSID1:u8 =0x02;
pub (crate) const CVI_MII_PHYSID2:u8 =0x03;

pub (crate) const CVI_MII_BMCR:u8 =0x00;
pub (crate) const CVI_BMCR_RESET:u16 =0x8000;

pub (crate) const ETH_PHY_BASE:usize = 0x03009000;
pub (crate) const ETH_PHY_INIT_MASK:usize = 0xFFFFFFF9;
pub (crate) const ETH_PHY_SHUTDOWN:usize = 1 << 1;
pub (crate) const ETH_PHY_POWERUP:usize = 0xFFFFFFFD;
pub (crate) const ETH_PHY_RESET:usize = 0xFFFFFFFB;
pub (crate) const ETH_PHY_RESET_N:usize = 1 << 2;
pub (crate) const ETH_PHY_LED_LOW_ACTIVE:usize = 1 << 3;