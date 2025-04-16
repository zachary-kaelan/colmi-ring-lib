use zerocopy::{Immutable, KnownLayout, FromBytes, byteorder::big_endian::*};

#[repr(C)]
#[derive(Copy, Clone, Debug, Immutable, KnownLayout, FromBytes)]
pub struct BloodO2 {
    command_id: u8,
    sample_type: u8,
    spo2: U16,
    spo2_max: U16,
    spo2_min: U16,
    spo2_diff: U16
}


#[repr(C)]
#[derive(Copy, Clone, Debug, Immutable, KnownLayout, FromBytes)]
pub struct PPG {
    command_id: u8,
    sample_type: u8,
    ppg: U16,
    ppg_max: U16,
    ppg_min: U16,
    ppg_diff: U16
}


#[repr(C)]
#[derive(Copy, Clone, Debug, Immutable, KnownLayout, FromBytes)]
pub struct AccelPacked {
    command_id: u8,
    sample_type: u8,
    data: [u8; 6]
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Accel {
    command_id: u8,
    sample_type: u8,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl AccelPacked {
    pub fn unpack(&self) -> Accel {
        let mut vals = [0f32; 3];
        for i in 0..3 {
            let msb = self.data[i * 2] as u16;
            let lsb = self.data[i * 2 + 1] as u16;
            let val: u16 = (msb << 4 | (lsb & 0xf)) & 0xfff;
            let val = if val & 2048 > 0 {
                i16::try_from(val).unwrap() - 4096
            } else {
                i16::try_from(val).unwrap()
            };
            vals[i] = f32::from(val) / 512.0;
        }

        Accel { command_id: self.command_id, sample_type: self.sample_type, x: vals[1], y: vals[2], z: vals[0] }
    }
}