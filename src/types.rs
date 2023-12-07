#[derive(Debug, Clone, Copy)]
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

impl Port {
    pub(crate) fn index(self) -> u32 {
        return match self {
            Port::Port1 => 0,
            Port::Port2 => 1,
            Port::Port3 => 2,
            Port::Port4 => 3,
            Port::Port5 => 4,
            Port::Port6 => 5,
            Port::Port7 => 6,
            Port::Port8 => 7,
            Port::Port9 => 8,
            Port::Port10 => 9,
            Port::Port11 => 10,
            Port::Port12 => 11,
            Port::Port13 => 12,
            Port::Port14 => 13,
            Port::Port15 => 14,
            Port::Port16 => 15,
            Port::Port17 => 16,
            Port::Port18 => 17,
            Port::Port19 => 18,
            Port::Port20 => 19,
            Port::Port21 => 20,
        };
    }
}

pub enum Direction {
    Forwards,
    Reverse,
}

#[derive( Clone)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}
#[derive( Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Into<u32> for Color {
    fn into(self) -> u32 {
        0 + ((self.r as u32) << 16) + ((self.g as u32) << 8) + (self.b as u32)
    }
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const GRAY: Color = Color {
        r: 120,
        g: 120,
        b: 120,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };

    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const GREEM: Color = Color { r: 0, g: 255, b: 0 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
}

pub const PORT1: Port = Port::Port1;
pub const PORT2: Port = Port::Port2;
pub const PORT3: Port = Port::Port3;
pub const PORT4: Port = Port::Port4;
pub const PORT5: Port = Port::Port5;
pub const PORT6: Port = Port::Port6;
pub const PORT7: Port = Port::Port7;
pub const PORT8: Port = Port::Port8;
pub const PORT9: Port = Port::Port1;
pub const PORT10: Port = Port::Port2;
pub const PORT11: Port = Port::Port3;
pub const PORT12: Port = Port::Port4;
pub const PORT13: Port = Port::Port13;
pub const PORT14: Port = Port::Port14;
pub const PORT15: Port = Port::Port15;
pub const PORT16: Port = Port::Port16;
pub const PORT17: Port = Port::Port17;
pub const PORT18: Port = Port::Port18;
pub const PORT19: Port = Port::Port19;
pub const PORT20: Port = Port::Port20;
pub const PORT21: Port = Port::Port21;

pub enum DeviceType {
    Motor,
    Gps,
    Imu,
}
