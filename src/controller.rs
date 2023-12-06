
use crate::api;

pub enum Type {
    Primary,
    Secondary,
}
impl Into<api::V5_ControllerId> for Type {
    fn into(self) -> api::V5_ControllerId {
        return match self {
            Self::Primary => api::_V5_ControllerId_kControllerMaster,
            Self::Secondary => api::_V5_ControllerId_kControllerPartner,
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

impl Into<api::V5_ControllerIndex> for Button {
    fn into(self) -> api::V5_ControllerId {
        return match self {
            Self::Up => api::_V5_ControllerIndex_ButtonUp,
            Self::Down => api::_V5_ControllerIndex_ButtonDown,
            Self::Left => api::_V5_ControllerIndex_ButtonLeft,
            Self::Right => api::_V5_ControllerIndex_ButtonRight,

            Self::A => api::_V5_ControllerIndex_ButtonA,
            Self::B => api::_V5_ControllerIndex_ButtonB,
            Self::X => api::_V5_ControllerIndex_ButtonX,
            Self::Y => api::_V5_ControllerIndex_ButtonY,

            Self::L1 => api::_V5_ControllerIndex_ButtonL1,
            Self::L2 => api::_V5_ControllerIndex_ButtonL2,
            Self::R1 => api::_V5_ControllerIndex_ButtonR1,
            Self::R2 => api::_V5_ControllerIndex_ButtonR2,
        };
    }
}

pub enum Axis {
    LeftX,
    LeftY,
    RightX,
    RightY,
}

impl Into<api::V5_ControllerIndex> for Axis {
    fn into(self) -> api::V5_ControllerId {
        return match self {
            Self::LeftX => api::_V5_ControllerIndex_AnaLeftX,
            Self::LeftY => api::_V5_ControllerIndex_AnaLeftY,
            Self::RightX => api::_V5_ControllerIndex_AnaRightX,
            Self::RightY => api::_V5_ControllerIndex_AnaRightY,
        };
    }
}

pub fn axis_value(controller: Type, a: Axis) -> f32 {
    unsafe {
        let val = api::vexControllerGet(controller.into(), a.into());
        return (val as f32) / 127.0;
    }
}
pub fn button_pressed(controller: Type, b: Button) -> bool {
    unsafe {
        let val = api::vexControllerGet(controller.into(), b.into());
        return val == 1;
    }
}