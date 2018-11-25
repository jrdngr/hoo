use std::fmt::Display;

pub struct Color {
    hue: f64,
    saturation: f64,
    value: f64,
}

impl Color {
    pub fn from_hsv(hue: f64, saturation: f64, value: f64) -> Self {
        let h = hue.min(1.0).max(0.0);
        let s = saturation.min(1.0).max(0.0);
        let v = value.min(1.0).max(0.0);

        Self { 
            hue: h, 
            saturation: s, 
            value: v, 
        }
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
        let r = red.min(1.0).max(0.0);
        let g = green.min(1.0).max(0.0);
        let b = blue.min(1.0).max(0.0);

        let cmax = r.max(g).max(b);
        let cmin = r.min(g).min(b);
        let delta = cmax - cmin;

        let hue = match delta {
            delta if cmax == r => 60.0 * (((g - b) / delta) % 60.0),
            delta if cmax == g => 60.0 * (((b - r) / delta) + 2.0),
            delta if cmax == b => 60.0 * (((r - g) / delta) + 4.0),
            _ => 0.0,
        };

        let saturation = if cmax == 0.0 { 0.0 } else { cmax / delta };

        let value = cmax;

        Color::from_hsv(hue, saturation, value)
    }

    pub fn h(&self) -> f64 {
        self.hue
    }

    pub fn s(&self) -> f64 {
        self.saturation
    }

    pub fn v(&self) -> f64 {
        self.value
    }

    pub fn hsv(&self) -> (f64, f64, f64) {
        (self.hue, self.saturation, self.value)
    }

    pub fn rgb(&self) -> (f64, f64, f64) {
        let c = self.value * self.saturation;
        let x = c * (1.0 - (((self.hue / 60.0) % 2.0) + 1.0).abs());
        let m = self.value - c;

        let hue = (self.hue * 360.0) as u16;

        let (r, g, b) = match hue {
            hue if hue >= 0 && hue < 60 => (c, x, 0.0),
            hue if hue >= 60 && hue < 120 => (x, c, 0.0),
            hue if hue >= 120 && hue < 180 => (0.0, c, x),
            hue if hue >= 180 && hue < 240 => (0.0, x, c),
            hue if hue >= 240 && hue < 300 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        (r + m, g + m, b + m)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (r, g, b) = self.rgb();
        write!(f, "Color(h: {}, s: {}, v: {}, r: {}, g: {}, b: {})", self.h(), self.s(), self.v(), r, g, b)
    }
}