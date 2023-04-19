## [0.4.0] Profiling & performance update
- Added: Support for profile tracing via the [profiling](https://crates.io/crates/profiling) crate.
- Changed: Calls to update_device_state now update the driver's device state in one step via a JOYSTICK_POSITION pointer. ~40 times faster than setting each button/axis/hat individually.

## [0.3.1] Config search patch
- Added: vJoy devices can be searched by their configuration (num_buttons, num_axes, num_hats).

## [0.3.0] Updated vJoy version
- Added: Support for up to 4 continuous hat switches in the range of 0..360° with a 1/100° resolution.
- Changed: Max number of axis increased to 16
- Changed: Supported vJoy driver version updated to 2.2.2.1 from: https://github.com/njz3/vJoy/

## [0.2.0] Hat switch support
- Added: Support for up to four 4-way hat switches (e.g. a D-pad).

## [0.1.0] Initial release