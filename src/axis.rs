use std::fmt::Display;

#[derive(Debug, Clone)]
/// Current state of an enabled device axis.
///
/// Range of IDs is 1..=8 for consistency with the .dll.
///
/// Table of IDs vs display names and hid usages:
///
/// | ID            |   1  |   2  |   3  |   4  |   5  |   6  |    7   |       8      |
/// |---------------|:----:|:----:|:----:|:----:|:----:|:----:|:------:|:------------:|
/// | Display name  |   X  |   Y  |   Z  |  Rx  |  Ry  |  Rz  | Slider | Dial/Slider2 |
/// | HID usage     | 0x30 | 0x31 | 0x32 | 0x33 | 0x34 | 0x35 |  0x36  |     0x37     |
pub struct Axis {
    pub(crate) id: u32,
    pub(crate) value: i32,
    pub(crate) display_name: String,
    pub(crate) hid_usage: u32,
}

impl Axis {
    pub fn get(&mut self) -> i32 {
        self.value
    }

    pub fn set(&mut self, value: i32) {
        self.value = value;
    }

    pub fn reset(&mut self) {
        self.value = i32::default();
    }

    pub fn display_name(&mut self) -> String {
        self.display_name.clone()
    }

    pub fn hid_usage(&mut self) -> u32 {
        self.hid_usage
    }
}

impl Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Axis ID: {} | value: {} | HID usage: {} | display name: {}",
            self.id, self.value, self.hid_usage, self.display_name
        ))
    }
}
