#[cfg(test)]

mod tests {
    // The process by which devices are acquired from the C API is not compatible with Rust's test harness and only works within the same test.
    // The tests can be monitored via the vJoyMonitor and vJoyList executables bundled with vJoy.
    use vjoy::{ButtonState, VJoy};

    #[test]
    fn test() {
        let mut vjoy = VJoy::from_default_dll_location().unwrap();
        let mut device_1 = vjoy.get_device_state(1).unwrap();

        // Simple test for 1 device
        println!("Setting button 1 of device 1");
        device_1.set_button(1, ButtonState::Pressed).unwrap();
        vjoy.update_device_state(&device_1).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));

        println!("Resetting button 1 of device 1");
        device_1.set_button(1, ButtonState::Released).unwrap();
        vjoy.update_device_state(&device_1).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));

        println!("Setting axis 1 of device 1 to i32::MAX");
        device_1.set_axis(1, i32::MAX).unwrap();
        vjoy.update_device_state(&device_1).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));

        println!("Setting axis 1 of device 1 to i32::MIN");
        device_1.set_axis(1, i32::MIN).unwrap();
        vjoy.update_device_state(&device_1).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
        // Simple test for 1 device

        // Test all buttons/axes for 1 device
        println!("Setting all buttons for device 1");
        for button in device_1.buttons_mut() {
            button.set(ButtonState::Pressed);
        }
        println!("Setting all axes to MAX");
        for axis in device_1.axes_mut() {
            axis.set(i32::MAX);
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        vjoy.update_device_state(&device_1).unwrap();

        println!("Resetting all buttons");
        for button in device_1.buttons_mut() {
            button.set(ButtonState::Released);
        }
        println!("Setting all axes to MIN");
        for axis in device_1.axes_mut() {
            axis.set(i32::MIN);
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        vjoy.update_device_state(&device_1).unwrap();
        // Test all buttons/axes for 1 device

        // Test all buttons/axes for all devices
        println!("Test all buttons/axes for all devices");
        for device in &mut vjoy.devices_cloned() {
            for button in device.buttons_mut() {
                button.set(ButtonState::Pressed);
            }
            for axis in device.axes_mut() {
                axis.set(i32::MAX);
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
            vjoy.update_device_state(&device).unwrap();

            for button in device.buttons_mut() {
                button.set(ButtonState::Released);
            }
            for axis in device.axes_mut() {
                axis.set(i32::MIN);
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
            vjoy.update_device_state(&device).unwrap();
        }
        // Test all buttons/axes for all devices

        // Rapid test all buttons/axes for all devices
        println!("Rapid test all buttons/axes for all devices");
        for _ in 0..1000 {
            for device in &mut vjoy.devices_cloned() {
                for button in device.buttons_mut() {
                    button.set(ButtonState::Pressed);
                }
                for axis in device.axes_mut() {
                    axis.set(i32::MAX);
                }
                vjoy.update_device_state(&device).unwrap();

                for button in device.buttons_mut() {
                    button.set(ButtonState::Released);
                }
                for axis in device.axes_mut() {
                    axis.set(i32::MIN);
                }
                vjoy.update_device_state(&device).unwrap();
            }
        }
        // Rapid test all buttons/axes for all devices
    }
}
