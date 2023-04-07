use crate::axis::Axis;
use crate::button::{Button, ButtonState};
use crate::device::Device;
use crate::error::{AppError, Error, FFIError};
use crate::{AXES_DISPLAY_NAMES, AXES_HID_USAGE};
use log::trace;
use vjoy_sys::VjdStat;

/// Main entry for this crate and controller for all vJoy devices.
///
/// [from_default_dll_location](Self::from_default_dll_location()) loads the vJoyInterface.dll from "C:/Program Files/vJoy/x64/vJoyInterface.dll".
/// Alternatively, you can provide a custom location via [from_dll_location](Self::from_dll_location()).
///
/// [get_device_state](Self::get_device_state()) returns the current state for a specific device.
/// You can alter this state and upload it to the virtual device via [update_device_state](Self::update_device_state()).
///
/// Note on IDs: Since the display names for the virtual devices and components are hardcoded into the .dll,
/// all IDs are one-based to avoid confusion.
/// ID-Ranges:
/// - Devices: 1..=16
/// - Buttons: 1..=128
/// - Axes: 1..=8
/// - Hat switches: 1..=4
/// ```no_run
/// # use vjoy::{VJoy, ButtonState, Error};///
/// let mut vjoy = VJoy::from_default_dll_location()?;
/// let mut device_1 = vjoy.get_device_state(1)?;
///
/// device_1.set_button(1, ButtonState::Pressed)?;
/// device_1.set_axis(1, i32::MAX)?;
///
/// vjoy.update_device_state(&device_1)?;
///
/// device_1.reset_all();
///
/// vjoy.update_device_state(&device_1)?;
/// # Ok::<(), Error>(())
/// ```
pub struct VJoy {
    ffi: vjoy_sys::vJoyInterface,
    devices: Vec<Device>,
}

impl VJoy {
    pub fn from_default_dll_location() -> Result<Self, Error> {
        Self::from_dll_location("C:/Program Files/vJoy/x64/vJoyInterface.dll")
    }

    pub fn from_dll_location(path: &str) -> Result<Self, Error> {
        let mut vjoy = Self::new(path)?;
        vjoy.fetch_devices();

        Ok(vjoy)
    }

    pub fn devices_cloned(&mut self) -> Vec<Device> {
        self.devices.clone()
    }

    pub fn get_device_state(&self, device_id: u32) -> Result<Device, Error> {
        match self
            .devices
            .binary_search_by(|device| device.id.cmp(&device_id))
        {
            Ok(index) => Ok(self.devices[index].clone()),
            Err(_) => Err(Error::App(AppError::DeviceNotFound(device_id))),
        }
    }

    pub fn update_device_state(&mut self, new_device_state: &Device) -> Result<(), Error> {
        let index = match self
            .devices
            .binary_search_by(|device| device.id.cmp(&new_device_state.id))
        {
            Ok(i) => i,
            Err(_) => return Err(Error::App(AppError::DeviceNotFound(new_device_state.id))),
        };

        {
            let device = self.devices.get_mut(index).unwrap();
            *device = new_device_state.clone();
        }

        let device = self.devices.get(index).unwrap();
        for button in &device.buttons {
            self.set_button(device.id, button.id, button.state)?;
        }

        for axis in &device.axes {
            self.set_axis(device.id, axis.id, axis.value)?;
        }

        Ok(())
    }

    fn fetch_devices(&mut self) {
        for device_id in 1..=16 {
            if self.acquire_device(device_id).is_ok() {
                let button_count = unsafe { self.ffi.GetVJDButtonNumber(device_id) } as u32;
                let buttons: Vec<Button> = (1..=button_count)
                    .map(|button_id| Button {
                        id: button_id as u8,
                        state: ButtonState::Released,
                    })
                    .collect();
                trace!("Device {} button count: {}", device_id, buttons.len());

                let mut axes = Vec::new();
                for axis_id in 1..=8 {
                    let axis_index = (axis_id - 1) as usize;
                    let axis_display_name = AXES_DISPLAY_NAMES[axis_index].to_string();
                    let axis_hid_usage = AXES_HID_USAGE[axis_index];
                    let exists = unsafe { self.ffi.GetVJDAxisExist(device_id, axis_hid_usage) };
                    trace!(
                        "Device {} axis id: {} display name: {} hid usage: {}",
                        device_id,
                        axis_id,
                        axis_display_name,
                        axis_hid_usage
                    );
                    if exists == 1 {
                        let axis = Axis {
                            display_name: axis_display_name,
                            hid_usage: axis_hid_usage,
                            id: axis_id,
                            value: 0,
                        };
                        axes.push(axis);
                    }
                }

                self.devices.push(Device {
                    id: device_id,
                    buttons,
                    axes,
                })
            }
        }
    }

    fn set_button(&self, device_id: u32, button_id: u8, state: ButtonState) -> Result<(), Error> {
        unsafe {
            let result = self.ffi.SetBtn(state as i32, device_id, button_id);
            if result != 1 {
                let device_state = self.get_device_ffi_status(device_id);
                return Err(Error::Ffi(FFIError::ButtonCouldNotBeSet(
                    device_id,
                    button_id,
                    device_state,
                )));
            }
        }
        Ok(())
    }

    fn set_axis(&self, device_id: u32, axis_id: u32, value: i32) -> Result<(), Error> {
        unsafe {
            let axis_index = (axis_id - 1) as usize;
            let axis_hid = AXES_HID_USAGE[axis_index];
            let result = self.ffi.SetAxis(value, device_id, axis_hid);
            if result != 1 {
                let device_state = self.get_device_ffi_status(device_id);
                return Err(Error::Ffi(FFIError::AxisCouldNotBeSet(
                    device_id,
                    axis_id,
                    device_state,
                )));
            }
        }
        Ok(())
    }

    fn acquire_device(&self, device_id: u32) -> Result<(), Error> {
        unsafe {
            let result = self.ffi.AcquireVJD(device_id);
            if result == 1 {
                trace!("Acquired device {}", device_id);
                Ok(())
            } else {
                Err(Error::Ffi(FFIError::DeviceCouldNotBeAcquired(
                    device_id, result,
                )))
            }
        }
    }

    fn relinquish_device(&self, device_id: u32) {
        unsafe {
            self.ffi.RelinquishVJD(device_id);
            println!("Relinquished device {}", device_id);
        }
    }

    fn get_device_ffi_status(&self, device_id: u32) -> VjdStat {
        unsafe { self.ffi.GetVJDStatus(device_id) }
    }

    fn new(path: &str) -> Result<Self, Error> {
        unsafe {
            let Ok(ffi) = vjoy_sys::vJoyInterface::new(path)
            else {
                return Err(Error::Ffi(FFIError::DynamicLybraryNotFound(path.to_string())));
            };

            Ok(Self {
                ffi,
                devices: Vec::new(),
            })
        }
    }
}

impl Drop for VJoy {
    fn drop(&mut self) {
        for device in &self.devices {
            self.relinquish_device(device.id);
        }
    }
}
