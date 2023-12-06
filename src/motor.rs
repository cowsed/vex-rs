use crate::types::Port;

pub struct Motor {
    pub port: Port,
}
impl Motor {
    pub fn new(_port: Port) -> Motor {
        todo!("make sure motor type aligns with what vex thinks is plugged in there");
    }
}

impl Motor {
    pub fn set_voltage(&self,_: f32) {
        todo!("set voltage")
    }
    pub fn get_rotations(&self) -> f32 {
        todo!("get rotations");
    }
}
