use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HSLColor {
    hue: u16,
    saturation: u16,
    lightness: u16,
    alpha: u16,
}

static RGB_MATCH: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"rgb\((\d+),(\d+),(\d+)\)"#).expect("Unable to compile regex: RGB_MATCH")
});

static RGBA_MATCH: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"rgba\((\d+),(\d+),(\d+),(\d+)\)"#).expect("Unable to compile regex: RGBA_MATCH")
});

static HSL_MATCH: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"hsl\((\d+),(\d+)%,(\d+)%\)"#).expect("Unable to compile regex: HSL_MATCH")
});

static HSLA_MATCH: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"hsla\((\d+),(\d+)%,(\d+)%,(\d+)%\)"#)
        .expect("Unable to compile regex: HSLA_MATCH")
});

impl HSLColor {
    pub fn set_hue(&mut self, hue: u16) {
        self.hue = hue;
    }

    pub fn set_lightness(&mut self, lightness: u16) {
        self.lightness = lightness;
    }

    pub fn set_saturation(&mut self, saturation: u16) {
        self.saturation = saturation;
    }

    pub fn set_alpha(&mut self, alpha: u16) {
        self.alpha = alpha;
    }

    pub fn get_hue(&self) -> &u16 {
        &self.hue
    }

    pub fn get_lightness(&self) -> &u16 {
        &self.lightness
    }

    pub fn get_saturation(&self) -> &u16 {
        &self.saturation
    }

    pub fn get_alpha(&self) -> &u16 {
        &self.alpha
    }

    pub fn new(hue: u16, saturation: u16, lightness: u16, alpha: u16) -> Self {
        Self {
            hue,
            saturation,
            lightness,
            alpha,
        }
    }

    pub fn as_normalized(&self) -> (f64, f64, f64, f64) {
        (
            self.hue as f64,
            (self.saturation as f64 / 100.0).clamp(0.0, 1.0),
            (self.lightness as f64 / 100.0).clamp(0.0, 1.0),
            (self.alpha as f64 / 100.0).clamp(0.0, 1.0),
        )
    }

    pub fn as_rgb(&self) -> (u8, u8, u8, u8) {
        let (h, s, l, a) = self.as_normalized();

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        (
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
            (a * 255.0) as u8,
        )
    }

    pub fn as_css_property(&self) -> String {
        format!(
            "hsla({}, {}%, {}%, {}%)",
            self.hue, self.saturation, self.lightness, self.alpha
        )
    }

    pub fn get_complementary(&self) -> HSLColor {
        let mut col = *self;
        col.hue = (col.hue + 180) % 360;
        col
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> HSLColor {
        let (r, g, b, a) = (r as f64, g as f64, b as f64, a as f64);
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        // Lightness
        let l = (max + min) / 2.0;

        // Saturation
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        } as u16;
        let l = l as u16;

        // Hue
        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let h = if h < 0.0 { h + 360.0 } else { h } as u16;

        // Alpha
        let a = (a / 255.0 * 100.0) as u16;

        HSLColor {
            hue: h,
            saturation: s,
            lightness: l,
            alpha: a,
        }
    }

    /// Tries to convert a CSS color to an internal representation. Currently supports rgb(),
    /// rgba(), hsl(), hsla(), and shortened rgb/rgba values.
    ///
    /// Please do not use this in anywhere performance is needed it uses like 4 different regex
    /// checks
    pub(crate) fn from_css_property(color: String) -> Option<Self> {
        let color = color
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        if HSLA_MATCH.is_match(&color) {
            let components = HSLA_MATCH.captures(&color).unwrap();
            let mut components = components.iter();
            components.next();
            let components = components
                .map(|c| c.unwrap().as_str().parse::<u16>().unwrap())
                .collect::<Vec<u16>>();
            Some(HSLColor::new(
                components[0],
                components[1],
                components[2],
                components[3],
            ))
        } else if HSL_MATCH.is_match(&color) {
            let components = HSL_MATCH.captures(&color).unwrap();
            let mut components = components.iter();
            components.next();
            let components = components
                .map(|c| c.unwrap().as_str().parse::<u16>().unwrap())
                .collect::<Vec<u16>>();
            Some(HSLColor::new(
                components[0],
                components[1],
                components[2],
                100,
            ))
        } else if RGB_MATCH.is_match(&color) {
            let components = RGB_MATCH.captures(&color).unwrap();
            let mut components = components.iter();
            components.next();
            let components = components
                .map(|c| c.unwrap().as_str().parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            Some(HSLColor::from_rgba(
                components[0],
                components[1],
                components[2],
                u8::MAX,
            ))
        } else if RGBA_MATCH.is_match(&color) {
            let components = RGBA_MATCH.captures(&color).unwrap();
            let mut components = components.iter();
            components.next();
            let components = components
                .map(|c| c.unwrap().as_str().parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            Some(HSLColor::from_rgba(
                components[0],
                components[1],
                components[2],
                components[3],
            ))
        } else if color.starts_with('#') {
            let mut c = Vec::new();

            let mut r = color.chars().skip(1).collect::<String>();
            let contains_alpha = r.len() != 6;
            let mut g = r.split_off(2);
            let mut b = g.split_off(2);

            c.push(u8::from_str_radix(&r, 16).unwrap());
            c.push(u8::from_str_radix(&g, 16).unwrap());

            if contains_alpha {
                let a = b.split_off(2);
                c.push(u8::from_str_radix(&b, 16).unwrap());
                c.push(u8::from_str_radix(&a, 16).unwrap());
            }
            c.push(u8::from_str_radix(&b, 16).unwrap());
            c.push(u8::MAX);

            Some(HSLColor::from_rgba(c[0], c[1], c[2], c[3]))
        } else {
            None
        }
    }
}

impl Default for HSLColor {
    fn default() -> Self {
        Self {
            hue: 0,
            saturation: 100,
            lightness: 100,
            alpha: 100,
        }
    }
}
