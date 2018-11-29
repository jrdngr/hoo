use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

use crate::color::Color;

pub type LightNumber = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightCollection(pub HashMap<LightNumber, Light>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Light {
    pub name: String,
    pub state: LightState,
}

impl Light {
    pub fn color(&self) -> Option<Color> {
        if let Some(hue) = self.state.hue {
            if let Some(saturation) = self.state.sat {
                if let Some(value) = self.state.bri {
                    return Some(Color::from_hsv(hue, saturation, value));
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LightState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bri: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sat: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xy: Option<(f32, f32)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<LightEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<LightAlert>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitiontime: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bri_inc: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sat_inc: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue_inc: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct_inc: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xy_inc: Option<(f32, f32)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colormode: Option<LightColorMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reachable: Option<bool>,
}

impl LightState {
    pub fn new() -> Self {
        Self {
            on: None,
            bri: None,
            hue: None,
            sat: None,
            xy: None,
            ct: None,
            effect: None,
            alert: None,
            transitiontime: None,
            bri_inc: None,
            sat_inc: None,
            hue_inc: None,
            ct_inc: None,
            xy_inc: None,
            colormode: None,
            reachable: None,
        }
    }

    pub fn on(mut self, is_on: bool) -> LightState {
        self.on = Some(is_on);
        self
    }

    pub fn bri(mut self, brightness: u8) -> LightState {
        self.bri = Some(brightness);
        self
    }

    pub fn hue(mut self, hue: u16) -> LightState {
        self.hue = Some(hue);
        self
    }

    pub fn sat(mut self, saturation: u8) -> LightState {
        self.sat = Some(saturation);
        self
    }

    pub fn xy(mut self, x: f32, y: f32) -> LightState {
        self.xy = Some((x, y));
        self
    }

    pub fn ct(mut self, color_temp: u16) -> LightState {
        self.ct = Some(color_temp);
        self
    }

    pub fn effect(mut self, effect: LightEffect) -> LightState {
        self.effect = Some(effect);
        self
    }

    pub fn alert(mut self, alert: LightAlert) -> LightState {
        self.alert = Some(alert);
        self
    }

    pub fn transitiontime(mut self, transition_time: u16) -> LightState {
        self.transitiontime = Some(transition_time);
        self
    }

    pub fn color(self, color: &Color) -> LightState {
        self.hue(color.h())
            .sat(color.s())
            .bri(color.v())
    }

    pub fn is_on(&self) -> bool {
        match self.on {
            Some(is_on) => is_on,
            None => false,
        }
    }

    pub fn is_reachable(&self) -> bool {
        match self.reachable {
            Some(is_reachable) => is_reachable,
            None => false,
        }
    }

    pub fn get_color(&self) -> Option<Color> {
        if self.hue.is_some() && self.sat.is_some() && self.bri.is_some() {
            Some(Color::from_hsv(self.hue.unwrap(), self.sat.unwrap(), self.bri.unwrap()))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum LightEffect {
    None,
    ColorLoop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum LightAlert {
    None,
    Select,
    Lselect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum LightColorMode {
    HS,
    XY,
    CT,
}