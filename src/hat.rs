#[derive(Debug, Clone, Copy)]
pub enum HatState {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
    Centered = 4,
}
impl Default for HatState {
    fn default() -> Self {
        Self::Centered
    }
}

/// Current state of an enabled device hat switch.
#[derive(Debug, Clone)]
pub struct Hat {
    pub(crate) id: u8,
    pub(crate) state: HatState,
}

impl Hat {
    pub fn get(&mut self) -> HatState {
        self.state
    }

    pub fn set(&mut self, state: HatState) {
        self.state = state;
    }

    pub fn reset(&mut self) {
        self.state = HatState::default();
    }
}
