#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub enum Type {
    Primary,
    Secondary,
}
impl Into<V5_ControllerId> for Type {
    fn into(self) -> V5_ControllerId {
        return match self {
            Self::Primary => _V5_ControllerId_kControllerMaster,
            Self::Secondary => _V5_ControllerId_kControllerPartner,
        };
    }
}
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    X,
    Y,
    L1,
    L2,
    R1,
    R2,
}

impl Into<V5_ControllerIndex> for Button {
    fn into(self) -> V5_ControllerId {
        return match self {
            Self::Up => _V5_ControllerIndex_ButtonUp,
            Self::Down => _V5_ControllerIndex_ButtonDown,
            Self::Left => _V5_ControllerIndex_ButtonLeft,
            Self::Right => _V5_ControllerIndex_ButtonRight,

            Self::A => _V5_ControllerIndex_ButtonA,
            Self::B => _V5_ControllerIndex_ButtonB,
            Self::X => _V5_ControllerIndex_ButtonX,
            Self::Y => _V5_ControllerIndex_ButtonY,

            Self::L1 => _V5_ControllerIndex_ButtonL1,
            Self::L2 => _V5_ControllerIndex_ButtonL2,
            Self::R1 => _V5_ControllerIndex_ButtonR1,
            Self::R2 => _V5_ControllerIndex_ButtonR2,
        };
    }
}

pub enum Axis {
    LeftX,
    LeftY,
    RightX,
    RightY,
}

impl Into<V5_ControllerIndex> for Axis {
    fn into(self) -> V5_ControllerId {
        return match self {
            Self::LeftX => _V5_ControllerIndex_AnaLeftX,
            Self::LeftY => _V5_ControllerIndex_AnaLeftY,
            Self::RightX => _V5_ControllerIndex_AnaRightX,
            Self::RightY => _V5_ControllerIndex_AnaRightY,
        };
    }
}

pub fn axis_value(controller: Type, a: Axis) -> f32 {
    unsafe {
        let val = vexControllerGet(controller.into(), a.into());
        return (val as f32) / 127.0;
    }
}
pub fn button_pressed(controller: Type, b: Button) -> bool {
    unsafe {
        let val = vexControllerGet(controller.into(), b.into());
        return val == 1;
    }
}