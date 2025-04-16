use zerocopy::{Immutable, KnownLayout, FromBytes};

#[repr(C)]
#[derive(Copy, Clone, Debug, Immutable, KnownLayout, FromBytes)]
pub struct SplitArrayHeader {
    command_id: u8,
    index: u8,
    len: u8,
    range: u8
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Immutable, KnownLayout, FromBytes)]
pub struct SplitArrayData {
    command_id: u8,
    index: u8,
    offset: u8,
    data: [u8; 12]
}