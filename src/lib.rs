#![no_std]
#![no_main]

pub mod types;
pub use types::{Color, Rect, Direction};
pub use types::{
    Port, PORT1, PORT10, PORT11, PORT12, PORT13, PORT14, PORT15, PORT16, PORT17, PORT18, PORT19,
    PORT2, PORT20, PORT21, PORT3, PORT4, PORT5, PORT6, PORT7, PORT8, PORT9,
};

pub mod competition;
pub mod controller;
pub mod motor;
pub mod screen;
pub mod time;
pub mod util;

mod api;
