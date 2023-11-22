//! Common traits and types for network device (NIC) drivers.

#![no_std]
#![feature(const_mut_refs)]
#![feature(const_slice_from_raw_parts_mut)]

mod net_buf;

use core::ptr::NonNull;

#[cfg(feature = "cviteknic")]
pub mod cviteknic;
#[cfg(feature = "cvitekphy")]
pub mod cvitekphy;

#[macro_use]
extern crate log;
extern crate alloc;
pub use cvitek_nic::{CvitekNicDevice, CvitekNicTraits, Packet, RxBuffer, TxBuffer};
pub use cvitek_phy::{CvitekPhyDevice, CvitekPhyTraits};

#[doc(no_inline)]
pub use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};

// pub use cvitek::CvitekNicTraits;
pub use self::net_buf::{NetBuf, NetBufBox, NetBufPool};

/// The ethernet address of the NIC (MAC address).
pub struct EthernetAddress(pub [u8; 6]);

/// Operations that require a network device (NIC) driver to implement.
pub trait NetDriverOps: BaseDriverOps {
    /// The ethernet address of the NIC.
    fn mac_address(&self) -> EthernetAddress;

    /// Whether can transmit packets.
    fn can_transmit(&self) -> bool;

    /// Whether can receive packets.
    fn can_receive(&self) -> bool;

    /// Size of the receive queue.
    fn rx_queue_size(&self) -> usize;

    /// Size of the transmit queue.
    fn tx_queue_size(&self) -> usize;

    /// Fills the receive queue with buffers.
    ///
    /// It should be called once when the driver is initialized.
    fn fill_rx_buffers(&mut self, buf_pool: &NetBufPool) -> DevResult;

    /// Prepares a buffer for transmitting.
    ///
    /// e.g., fill the header of the packet.
    fn prepare_tx_buffer(&self, tx_buf: &mut NetBuf, packet_len: usize) -> DevResult;

    /// Gives back the `rx_buf` to the receive queue for later receiving.
    ///
    /// `rx_buf` should be the same as the one returned by
    /// [`NetDriverOps::receive`].
    fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult;

    /// Poll the transmit queue and gives back the buffers for previous transmiting.
    /// returns [`DevResult`].
    fn recycle_tx_buffers(&mut self) -> DevResult;

    /// Transmits a packet in the buffer to the network, without blocking,
    /// returns [`DevResult`].
    fn transmit(&mut self, tx_buf: TxBuf) -> DevResult;

    /// Receives a packet from the network and store it in the [`NetBuf`],
    /// returns the buffer.
    ///
    /// Before receiving, the driver should have already populated some buffers
    /// in the receive queue by [`NetDriverOps::recycle_rx_buffer`].
    ///
    /// If currently no incomming packets, returns an error with type
    /// [`DevError::Again`].
    fn receive(&mut self) -> DevResult<RxBuf>;

    /// Allocate a memory buffer of a specified size for network transmission,
    /// returns [`DevResult`]
    fn alloc_tx_buffer(&self, size: usize) -> DevResult<TxBuf>;
}

pub trait PhyDriverOps: BaseDriverOps{
    /// configure phy register
    fn configure(&self);
    /// start phy transmit
    fn start(&self);
    /// stop phy transmit
    fn stop(&self);
}
pub enum TxBuf {
    CvitekNic(TxBuffer),
    Virtio(NetBufPtr),
}

impl TxBuf {
    pub fn packet(&self) -> &[u8] {
        match self {
            Self::CvitekNic(tx_buf) => tx_buf.packet(),
            Self::Virtio(tx_buf) => tx_buf.packet(),
        }
    }

    pub fn packet_mut(&mut self) -> &mut [u8] {
        match self {
            Self::CvitekNic(tx_buf) => tx_buf.packet_mut(),
            Self::Virtio(tx_buf) => tx_buf.packet_mut(),
        }
    }
}

pub enum RxBuf {
    CvitekNic(RxBuffer),
    Virtio(NetBufPtr),
}

impl RxBuf {
    pub fn packet(&self) -> &[u8] {
        match self {
            Self::CvitekNic(rx_buf) => rx_buf.packet(),
            Self::Virtio(rx_buf) => rx_buf.packet(),
        }
    }

    pub fn packet_mut(&mut self) -> &mut [u8] {
        match self {
            Self::CvitekNic(rx_buf) => rx_buf.packet_mut(),
            Self::Virtio(rx_buf) => rx_buf.packet_mut(),
        }
    }
}

/// A raw buffer struct for network device.
pub struct NetBufPtr {
    // The raw pointer of the original object.
    raw_ptr: NonNull<u8>,
    // The pointer to the net buffer.
    buf_ptr: NonNull<u8>,
    len: usize,
}

impl NetBufPtr {
    /// Create a new [`NetBufPtr`].
    pub fn new(raw_ptr: NonNull<u8>, buf_ptr: NonNull<u8>, len: usize) -> Self {
        Self {
            raw_ptr,
            buf_ptr,
            len,
        }
    }

    /// Return raw pointer of the original object.
    pub fn raw_ptr<T>(&self) -> *mut T {
        self.raw_ptr.as_ptr() as *mut T
    }

    /// Return [`NetBufPtr`] buffer len.
    pub fn packet_len(&self) -> usize {
        self.len
    }

    /// Return [`NetBufPtr`] buffer as &[u8].
    pub fn packet(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.buf_ptr.as_ptr() as *const u8, self.len) }
    }

    /// Return [`NetBufPtr`] buffer as &mut [u8].
    pub fn packet_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.buf_ptr.as_ptr(), self.len) }
    }
}
