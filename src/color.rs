#![allow(clippy::many_single_char_names)]

use std::fmt::Display;

pub struct Color {
    hue: u16,
    saturation: u8,
    value: u8,
}

impl Color {
    pub fn from_hsv(hue: u16, saturation: u8, value: u8) -> Self {
        Self { hue, saturation, value }
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
        let r = red.min(1.0).max(0.0);
        let g = green.min(1.0).max(0.0);
        let b = blue.min(1.0).max(0.0);

        let cmax = r.max(g).max(b);
        let cmin = r.min(g).min(b);
        let delta = cmax - cmin;

        let error: f64 = 0.0001;

        let mut hue = match cmax {
            cmax if (cmax - r) < error => 60.0 * ((g - b) / delta),
            cmax if (cmax - g) < error => 60.0 * (((b - r) / delta) + 2.0),
            cmax if (cmax - b) < error=> 60.0 * (((r - g) / delta) + 4.0),
            _ => 0.0,
        };

        if hue < 0.0 {
            hue += 360.0;
        }

        let saturation = if (cmax - 0.0) < error || (cmax - 1.0) < error { 0.0 } else { delta / cmax };

        let value = cmax;

        let h = ((hue / 360.0) * f64::from(std::u16::MAX)) as u16;
        let s = (saturation * f64::from(std::u8::MAX)) as u8;
        let v = (value * f64::from(std::u8::MAX)) as u8;

        Color::from_hsv(h, s, v)
    }

    pub fn h(&self) -> u16 {
        self.hue
    }

    pub fn s(&self) -> u8 {
        self.saturation
    }

    pub fn v(&self) -> u8 {
        self.value
    }

    pub fn hsv(&self) -> (u16, u8, u8) {
        (self.hue, self.saturation, self.value)
    }

    pub fn rgb(&self) -> (f64, f64, f64) {
        let h = (f64::from(self.hue) / f64::from(std::u16::MAX)) * 360.0;
        let s = f64::from(self.saturation) / f64::from(std::u8::MAX);
        let v = f64::from(self.value) / f64::from(std::u8::MAX);

        let c = v * s;
        let hp = h / 60.0;
        let x = c * (1.0 - ((hp % 2.0) - 1.0).abs());

        let (r, g, b) = match hp {
            hp if hp >= 0.0 && hp <= 1.0 => (c, x, 0.0),
            hp if hp > 1.0 && hp <= 2.0  => (x, c, 0.0),
            hp if hp > 2.0 && hp <= 3.0  => (0.0, c, x),
            hp if hp > 3.0 && hp <= 4.0  => (0.0, x, c),
            hp if hp > 4.0 && hp <= 5.0  => (x, 0.0, c),
            hp if hp > 5.0 && hp <= 6.0  => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0)
        };
   
        let m = v - c;

        (r + m, g + m, b + m)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (r, g, b) = self.rgb();
        write!(f, "Color(h: {}, s: {}, v: {}, r: {}, g: {}, b: {})", self.h(), self.s(), self.v(), r, g, b)
    }
}

pub fn deg_to_u16(deg: f64) -> u16 {
    let multiplier = deg / 360.0;
    (multiplier * f64::from(std::u16::MAX)) as u16
}