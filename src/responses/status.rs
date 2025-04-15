use zerocopy::{Immutable, FromBytes, KnownLayout};

#[repr(C)]
#[derive(Copy, Clone, Debug, Immutable, KnownLayout, FromBytes)]
pub struct SetTimeResponse {
    command_id: u8,
    supports_temperature: u8,
    supports_plate: u8,
    supports_menstruation: u8,
    support_flags1: u8,
    width: u16,
    height: u16,
    use_new_sleep_protocol: u8,
    max_watchface: u8,
    support_flags2: u8,
    support_flags3: u8,
    max_contacts: u8,
    support_flags4: u8,
}