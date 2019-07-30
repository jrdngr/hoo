use hoo_api::{LightCollection, LightNumber, LightState};

use crate::animation::dynamic::Operation;

#[derive(Default)]
pub struct LightStateTransform {
    pub on: Option<Operation<bool>>,
    pub transition_time: Option<Operation<u16>>,
    pub hue: Option<Operation<u16>>,
    pub saturation: Option<Operation<u8>>,
    pub brightness: Option<Operation<u8>>,
}

impl LightStateTransform {
    pub fn create_light_state(
        &mut self,
        light_num: LightNumber,
        previous_states: &LightCollection,
    ) -> LightState {
        let previous_state = previous_states.0.get(&light_num);

        let on = self.on.as_mut().and_then(|op| {
            op.apply(
                previous_states,
                previous_state.and_then(|light| light.state.on),
            )
        });

        let transitiontime = self.transition_time.as_mut().and_then(|op| {
            op.apply(
                previous_states,
                previous_state.and_then(|light| light.state.transitiontime),
            )
        });

        let hue = self.hue.as_mut().and_then(|op| {
            op.apply(
                previous_states,
                previous_state.and_then(|light| light.state.hue),
            )
        });

        let sat = self.saturation.as_mut().and_then(|op| {
            op.apply(
                previous_states,
                previous_state.and_then(|light| light.state.sat),
            )
        });

        let bri = self.brightness.as_mut().and_then(|op| {
            op.apply(
                previous_states,
                previous_state.and_then(|light| light.state.bri),
            )
        });

        LightState {
            on,
            transitiontime,
            hue,
            sat,
            bri,
            ..Default::default()
        }
    }
}
