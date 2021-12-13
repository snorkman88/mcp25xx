use core::fmt::Debug;

use embedded_can::{Frame, Id};

use crate::registers::DLC;
use crate::IdHeader;

/// CAN frame with the same in memory representation as the registers of the CAN controller
#[derive(Clone, Default)]
#[repr(C)]
pub struct CanFrame {
    pub(crate) id_header: IdHeader,
    pub(crate) dlc: DLC,
    pub(crate) data: [u8; 8],
}

impl CanFrame {
    pub(crate) fn as_bytes(&self) -> &[u8; 13] {
        // SAFETY:
        // Frame is [repr(C)] without any padding bytes
        unsafe { &*(self as *const CanFrame as *const [u8; core::mem::size_of::<CanFrame>()]) }
    }
}

impl Frame for CanFrame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Result<Self, ()> {
        if data.len() > 8 {
            return Err(());
        }

        let mut frame = CanFrame {
            id_header: IdHeader::from(id.into()),
            dlc: DLC::new().with_dlc(data.len() as u8),
            data: [0; 8],
        };

        frame.data[..data.len()].copy_from_slice(data);

        Ok(frame)
    }

    fn new_remote(id: impl Into<Id>, dlc: usize) -> Result<Self, ()> {
        if dlc > 8 {
            return Err(());
        }
        Ok(CanFrame {
            id_header: IdHeader::from(id.into()),
            dlc: DLC::new().with_dlc(dlc as u8).with_rtr(true),
            data: [0; 8],
        })
    }

    #[inline]
    fn is_extended(&self) -> bool {
        self.id_header.exide()
    }

    #[inline]
    fn is_remote_frame(&self) -> bool {
        self.dlc.rtr()
    }

    fn id(&self) -> Id {
        self.id_header.id()
    }

    #[inline]
    fn dlc(&self) -> usize {
        self.dlc.dlc() as usize
    }

    #[inline]
    fn data(&self) -> &[u8] {
        &self.data[0..self.dlc()]
    }
}

impl Debug for CanFrame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CanFrame")
            .field(
                "id",
                match &self.id() {
                    Id::Standard(id) => id,
                    Id::Extended(id) => id,
                },
            )
            .field("is_remote_frame", &self.is_remote_frame())
            .field("data", &self.data())
            .finish()
    }
}
