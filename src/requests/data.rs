use zerocopy::{Immutable, IntoBytes};

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Immutable, IntoBytes)]
pub enum DataType {
    HeartRate = 1,
    BloodPressure = 2,
    BloodOxygen = 3,
    Fatigue = 4,
    HealthCheck = 5,
    RealtimeHeartRate = 6,
    ECG = 7,
    Pressure = 8,
    BloodSugar = 9,
    HRV = 10
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Immutable, IntoBytes)]
pub enum DataAction {
    Start = 1,
    Pause = 2,
    Continue = 3,
    Stop = 4
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Immutable, IntoBytes)]
pub struct DataRequest {
    command_id: u8,
    data_type: u8,
    data_action: DataAction
}

impl DataRequest {
    pub fn new(data_type: u8, data_action: DataAction) -> DataRequest {
        DataRequest { command_id: 105, data_type, data_action }
    }
}