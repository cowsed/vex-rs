use crate::Port;

pub struct Motor {
    pub port: Port,
}
impl Motor {
    pub fn new(port: Port) -> Motor {
        todo!("make sure motor type aligns with what vex thinks is plugged in there");
    }
}

impl Motor {
    pub fn set_voltage(_: f32) {
        todo!("set voltage")
    }
    pub fn get_rotations() -> f32 {
        todo!("get rotations");
    }
}
