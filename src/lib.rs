#![doc = include_str!("../README.md")]

mod vjoy;
pub use crate::vjoy::VJoy;

mod error;
pub use error::*;

mod device;
pub use device::Device;

mod axis;
pub use axis::Axis;

mod hat;
pub use hat::Hat;

mod button;
pub use button::{Button, ButtonState};

const AXES_DISPLAY_NAMES: [&str; 8] = ["X", "Y", "Z", "Rx", "Ry", "Rz", "Slider", "Dial/Slider2"];
const AXES_HID_USAGE: [u32; 8] = [0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37];
