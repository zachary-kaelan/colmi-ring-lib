use zerocopy::{Immutable, IntoBytes};
use crate::requests::DataType;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Immutable, IntoBytes)]
pub enum RawDataAction {
    Disable = 2,
    Enable = 4,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Immutable, IntoBytes)]
pub enum RawDataType {
    All = 0,
    UNK1 = 1,
    UNK2 = 2,
    UNK3 = 3,
    Accel = 4,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Immutable, IntoBytes)]
pub struct RawDataRequest {
    command_id: u8,
    data_action: RawDataAction,
    data_type: RawDataType,
}

impl RawDataRequest {
    pub fn new(data_action: RawDataAction, data_type: RawDataType) -> RawDataRequest {
        RawDataRequest { command_id: 161, data_action, data_type }
    }
}