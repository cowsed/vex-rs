pub enum Port {
    Port1,
    Port2,
    Port3,
    Port4,
    Port5,
    Port6,
    Port7,
    Port8,
    Port9,
    Port10,
    Port11,
    Port12,
    Port13,
    Port14,
    Port15,
    Port16,
    Port17,
    Port18,
    Port19,
    Port20,
    Port21,
}
pub struct Motor {
    port: Port,
}
impl Motor{
    fn new() -> Motor{
        todo!("make sure motor type aligns with what vex thinks is plugged in there");
    }
}

impl Motor{
    fn set_voltage(volts: f32){
        todo!("set voltage")
    }
    fn get_rotations() -> f32{
        todo!("get rotations");
        return 0.0
    }
}