#[derive(Debug, Clone)]
/// Current state of an enabled device hat switch.
pub struct Hat {
    pub(crate) id: u8,
    pub(crate) value: u32,
}

impl Hat {
    pub fn get(&mut self) -> u32 {
        self.value
    }

    pub fn set(&mut self, value: u32) {
        self.value = value;
    }

    pub fn reset(&mut self) {
        self.value = u32::default();
    }
}
