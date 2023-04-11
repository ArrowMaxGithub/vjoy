#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum FourWayHat {
    Centered = -1,
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

/// Common state for either a 4-way hat or a continuous 360° hat switch
#[derive(Debug, Clone, Copy)]
pub enum HatState {
    Discrete(FourWayHat),
    Continuous(u32),
}

impl HatState {
    fn reset(&mut self) {
        *self = match self {
            HatState::Discrete(_) => HatState::Discrete(FourWayHat::Centered),
            HatState::Continuous(_) => HatState::Continuous(u32::MAX),
        };
    }
}

/// Current state of an enabled device hat switch.
///
/// A vJoy hat switch is either a 4-way discrete switch or a continuous switch. Continuous switches feature a range of 360° with a 1/100° resolution.
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
        self.state.reset();
    }
}
