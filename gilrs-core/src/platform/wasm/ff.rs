// Copyright 2016-2018 Mateusz Sieczko and other GilRs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::time::Duration;

#[derive(Debug)]
pub struct Device(web_sys::GamepadHapticActuator);

impl Device {
    pub(crate) fn new(gp_haptic: web_sys::GamepadHapticActuator) -> Self {
        Self(gp_haptic)
    }

    pub fn set_ff_state(&mut self, strong: u16, weak: u16, min_duration: Duration) {
        const MAX_U16: f64 = u16::MAX as f64;

        let gamepad_effect_parameters = web_sys::GamepadEffectParameters::new();
        gamepad_effect_parameters.set_duration(min_duration.as_secs_f64() * 1_000.0);
        gamepad_effect_parameters.set_strong_magnitude((strong as f64) / MAX_U16);
        gamepad_effect_parameters.set_weak_magnitude((weak as f64) / MAX_U16);

        let _ = self.0.play_effect_with_params(
            web_sys::GamepadHapticEffectType::DualRumble,
            &gamepad_effect_parameters,
        );
    }
}
