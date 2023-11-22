#![no_std]
#![allow(dead_code)]

extern crate alloc;
#[macro_use]
extern crate log;
mod cvitek_defs;
mod cvitek_phy_dev;
pub use cvitek_phy_dev::CvitekPhyDevice;
pub use cvitek_phy_dev::CvitekPhyTraits;