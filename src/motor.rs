use crate::api;
use crate::types::Port;
pub struct Motor {
    port: Port,
}

impl Motor {
    pub fn new(port: Port) -> Motor {
        Motor { port }
        // todo!("make sure motor type aligns with what vex thinks is plugged in there and update map of motors so we can yell when there are more than one device on the same port");
    }
}

impl Motor {
    pub fn reverse(self) -> Self {
        unsafe {
            let index = self.port.index() as u32;
            api::vexDeviceMotorReverseFlagSet(
                api::vexDeviceGetByIndex(index),
                !api::vexDeviceMotorReverseFlagGet(api::vexDeviceGetByIndex(index)),
            );
        }
        return self;
    }
    pub fn set_voltage(&self, volts: f32) {
        unsafe {
            let modifier = 20000.0 / 12.0;
            let value = (volts * modifier) as i32; // convert from volts to i32s
            api::vexDeviceMotorVoltageSet(
                api::vexDeviceGetByIndex(self.port.index() as u32),
                value,
            );
        }
    }
    pub fn get_rotations(&self) -> f32 {
        todo!("get rotations");
    }
}
