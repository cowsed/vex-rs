use crate::api;

use core::fmt::Display;
#[derive(Clone, Copy)]
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
impl Display for Port {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Port::Port1 => "Port1",
            Port::Port2 => "Port2",
            Port::Port3 => "Port3",
            Port::Port4 => "Port4",
            Port::Port5 => "Port5",
            Port::Port6 => "Port6",
            Port::Port7 => "Port7",
            Port::Port8 => "Port8",
            Port::Port9 => "Port9",
            Port::Port10 => "Port10",
            Port::Port11 => "Port11",
            Port::Port12 => "Port12",
            Port::Port13 => "Port13",
            Port::Port14 => "Port14",
            Port::Port15 => "Port15",
            Port::Port16 => "Port16",
            Port::Port17 => "Port17",
            Port::Port18 => "Port18",
            Port::Port19 => "Port19",
            Port::Port20 => "Port20",
            Port::Port21 => "Port21",
        };
        write!(f, "{}", s)
    }
}

impl Port {
    pub(crate) fn index(self) -> i32 {
        unsafe {
            return match self {
                Port::Port1 => api::vex_PORT1,
                Port::Port2 => api::vex_PORT2,
                Port::Port3 => api::vex_PORT3,
                Port::Port4 => api::vex_PORT4,
                Port::Port5 => api::vex_PORT5,
                Port::Port6 => api::vex_PORT6,
                Port::Port7 => api::vex_PORT7,
                Port::Port8 => api::vex_PORT8,
                Port::Port9 => api::vex_PORT9,
                Port::Port10 => api::vex_PORT10,
                Port::Port11 => api::vex_PORT11,
                Port::Port12 => api::vex_PORT12,
                Port::Port13 => api::vex_PORT13,
                Port::Port14 => api::vex_PORT14,
                Port::Port15 => api::vex_PORT15,
                Port::Port16 => api::vex_PORT16,
                Port::Port17 => api::vex_PORT17,
                Port::Port18 => api::vex_PORT18,
                Port::Port19 => api::vex_PORT19,
                Port::Port20 => api::vex_PORT20,
                Port::Port21 => api::vex_PORT21,
            };
        }
    }
}

pub enum Direction {
    Forwards,
    Reverse,
}

#[derive(Clone)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}
#[derive(Clone, Copy, PartialEq, Eq)]
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
