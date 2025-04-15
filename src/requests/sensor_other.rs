use zerocopy::{Immutable, IntoBytes};

#[repr(C)]
#[derive(Copy, Clone, Debug, Immutable, IntoBytes)]
pub struct PressureRequest {
    command_id: u8,
    index: u8
}

impl PressureRequest {
    pub fn new(index: u8) -> PressureRequest {
        PressureRequest { command_id: 55, index }
    }
}