use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::color::Color;

pub type LightNumber = u8;
pub type LightCollection = HashMap<LightNumber, Light>;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        Default::default()
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
        self.hue(color.hue).sat(color.saturation).bri(color.value)
    }

    pub fn reset_advanced(&mut self) {
        self.xy = None;
        self.ct = None;
        self.colormode = None;
        self.alert = None;
        self.effect = None;
        self.reachable = None;
        self.bri_inc = None;
        self.sat_inc = None;
        self.hue_inc = None;
        self.ct_inc = None;
        self.xy_inc = None;
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
        if let (Some(hue), Some(sat), Some(bri)) = (self.hue, self.sat, self.bri) {
            Some(Color::from_hsv(hue, sat, bri))
        } else {
            None
        }
    }

    pub fn combine(base: &Self, diff: &Self) -> Self {
        let on = diff.on.or(base.on);
        let bri = diff.bri.or(base.bri);
        let hue = diff.hue.or(base.hue);
        let sat = diff.sat.or(base.sat);
        let xy = diff.xy.or(base.xy);
        let ct = diff.ct.or(base.ct);
        let effect = diff.effect.or(base.effect);
        let alert = diff.alert.or(base.alert);
        let transitiontime = diff.transitiontime.or(base.transitiontime);
        let bri_inc = diff.bri_inc.or(base.bri_inc);
        let sat_inc = diff.sat_inc.or(base.sat_inc);
        let hue_inc = diff.hue_inc.or(base.hue_inc);
        let ct_inc = diff.ct_inc.or(base.ct_inc);
        let xy_inc = diff.xy_inc.or(base.xy_inc);
        let colormode = diff.colormode.or(base.colormode);
        let reachable = diff.reachable.or(base.reachable);

        Self {
            on,
            bri,
            hue,
            sat,
            xy,
            ct,
            effect,
            alert,
            transitiontime,
            bri_inc,
            sat_inc,
            hue_inc,
            ct_inc,
            xy_inc,
            colormode,
            reachable,
        }
    }
}

impl Default for LightState {
    fn default() -> Self {
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
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LightEffect {
    None,
    ColorLoop,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LightAlert {
    None,
    Select,
    Lselect,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LightColorMode {
    HS,
    XY,
    CT,
}
