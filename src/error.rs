use thiserror::Error;
use vjoy_sys::VjdStat;

#[derive(Error, Debug)]
pub enum Error {
    #[error("app error: {0}.")]
    App(AppError),

    #[error("ffi error: {0}.")]
    Ffi(FFIError),
}

#[derive(Error, Debug)]
/// Errors that arise from incorrect usage of the wrapper API - e.g. requesting buttons outside reported limits.
pub enum AppError {
    #[error("device with ID {0} was not found.")]
    DeviceNotFound(u32),

    #[error("axis {1} of Device {0} could not be found.")]
    AxisNotFound(u32, u32),

    #[error("button {1} of Device {0} could not be found.")]
    ButtonNotFound(u32, u8),
}

#[derive(Error, Debug)]
/// Errors that arise from incorrect usage of the C API - e.g. updating devices before acquisition.
pub enum FFIError {
    #[error("vJoyInterface.dll was not found at {0}")]
    DynamicLybraryNotFound(String),

    #[error("device with ID {0} could not be acquired. Device Status: {1}")]
    DeviceCouldNotBeAcquired(u32, VjdStat),

    #[error("button {1} of Device {0} could not be set. Device Status: {2}")]
    ButtonCouldNotBeSet(u32, u8, VjdStat),

    #[error("hat {1} of Device {0} could not be set. Device Status: {2}")]
    HatCouldNotBeSet(u32, u8, VjdStat),

    #[error("axis {1} of Device {0} could not be set. Device Status: {2}")]
    AxisCouldNotBeSet(u32, u32, VjdStat),
}
