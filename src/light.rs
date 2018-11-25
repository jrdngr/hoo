use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Light {
    pub name: String,
    pub state: LightState,
}

#[derive(Debug, Deserialize)]
pub struct LightState {
    pub on: Option<bool>,
    pub bri: Option<u8>,
    pub hue: Option<u16>,
    pub sat: Option<u8>,
    pub xy: Option<(f32, f32)>,
    pub ct: Option<u16>,
    pub effect: Option<LightEffect>,
    pub alert: Option<LightAlert>,
    pub transitiontime: Option<u16>,
    pub bri_inc: Option<i16>,
    pub sat_inc: Option<i16>,
    pub hue_inc: Option<i32>,
    pub ct_inc: Option<i32>,
    pub xy_inc: Option<(f32, f32)>,
    pub colormode: Option<LightColorMode>,
    pub reachable: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum LightEffect {
    None,
    ColorLoop,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum LightAlert {
    None,
    Select,
    Lselect,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum LightColorMode {
    HS,
    XY,
    CT,
}