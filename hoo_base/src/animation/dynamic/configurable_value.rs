use hoo_api::{LightCollection, LightNumber};

use crate::animation::dynamic::BoxedValueProducer;

pub enum ConfigurableValue {
    On(BoxedValueProducer<bool>),
    Hue(BoxedValueProducer<u16>),
    Saturation(BoxedValueProducer<u8>),
    Brightness(BoxedValueProducer<u8>),
    TransitionTime(BoxedValueProducer<u16>),
}

impl ConfigurableValue {
    pub fn apply(&mut self, lights: &mut LightCollection, light_num: LightNumber) {
        use ConfigurableValue::*;

        if let Some(light) = lights.get_mut(&light_num) {
            light.state.reset_advanced();

            match self {
                On(val) => light.state.on = Some(val.produce()),
                Hue(val) => light.state.hue = Some(val.produce()),
                Saturation(val) => light.state.sat = Some(val.produce()),
                Brightness(val) => light.state.bri = Some(val.produce()),
                TransitionTime(val) => light.state.transitiontime = Some(val.produce()),
            }
        };
    }
}
