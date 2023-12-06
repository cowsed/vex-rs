use crate::api;
use crate::types::Port;
pub struct Motor {
    port: Port,
}

impl Motor {
    pub fn new(_port: Port) -> Motor {
        todo!("make sure motor type aligns with what vex thinks is plugged in there and update map of motors so we can yell when there are more than one device on the same port");

    }
}

impl Motor {
    pub fn reverse(self) -> Self {
        unsafe {
            api::vexDeviceMotorReverseFlagSet(
                api::vexDeviceGetByIndex(self.port.index()),
                !api::vexDeviceMotorReverseFlagGet(api::vexDeviceGetByIndex(self.port.index())),
            );
        }
        return self;
    }
    pub fn set_voltage(&self, _: f32) {
        todo!("set voltage")
    }
    pub fn get_rotations(&self) -> f32 {
        todo!("get rotations");
    }
}
