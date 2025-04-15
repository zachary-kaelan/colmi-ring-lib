use zerocopy::{Immutable, IntoBytes};

#[repr(C)]
#[derive(Copy, Clone, Debug, Immutable, IntoBytes)]
pub struct BatteryRequest {
    command_id: u8
}

impl BatteryRequest {
    pub fn new() -> BatteryRequest {
        BatteryRequest { command_id: 3 }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Immutable, IntoBytes)]
pub struct RebootRequest {
    command_id: u8,
    constant: u8
}

impl RebootRequest {
    pub fn new() -> RebootRequest {
        RebootRequest { command_id: 8, constant: 1 }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Immutable, IntoBytes)]
pub struct SetTimeRequest {
    command_id: u8,
    year: u8,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl SetTimeRequest {
    pub fn new(year: u8,
               month: u8,
               day: u8,
               hour: u8,
               minute: u8,
               second: u8) -> SetTimeRequest {
        SetTimeRequest { command_id: 1, year, month, day, hour, minute, second, }
    }
}