use std::f32::consts::PI;

use crate::DirectionVector;
use web_sys;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub const RADIUS: f32 = 200.0;
pub const MAX_ANGLE: f32 = PI / 4.0;
pub const SPEED: f32 = 2.0;
pub const SPEED_MULTIPLIER: f32 = 1.1;
pub const HORIZONTAL_BOUNCE: f32 = 1.0;
pub const VERTICAL_BOUNCE: f32 = 2.0;

pub fn fast_inv_sqrt(x: f32) -> f32 {
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);
    y * (1.5 - 0.5 * x * y * y)
}

pub fn wasm_log(str: String) {
    web_sys::console::log_1(&str.into());
}
