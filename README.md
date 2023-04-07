Safe and idiomatic wrapper for [vjoy-sys](https://crates.io/crates/vjoy-sys).

# About vJoy
vJoy simulates up to 16 input devices with up to 128 buttons, 8 axes, and 4 hat switches.
The virtual devices can be used to 
1) Emulate gamepads/joysticks for older games that require a specific kind of input.
2) Combine multiple physical devices into one virtual.
3) Apply transformations from a physical device to a virtual device (e.g. 2-button to axis rebind, software filtering etc.).

The virtual devices appear to applications as regular input devices.

# Usage
The [vJoy driver](https://sourceforge.net/projects/vjoystick/) needs to be installed and is only available for Windows.

The vJoy shared library is loaded at runtime via libloading. See the integration tests for specifics.

# Example
```no_run
use vjoy::{VJoy, ButtonState, Error};

fn main() -> Result<(), Error>{
    let mut vjoy = VJoy::from_default_dll_location()?;
    let mut device_1 = vjoy.get_device_state(1)?;

    device_1.set_button(1, ButtonState::Pressed)?;
    device_1.set_axis(1, i32::MAX)?;
    vjoy.update_device_state(&device_1)?;

    device_1.reset_all();
    vjoy.update_device_state(&device_1)?;

    Ok(())
}
```

# Planned features
- Support for hat switches
- Rust native input viewer via egui
