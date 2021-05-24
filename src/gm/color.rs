#[derive(Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }

    pub fn make(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b, a: 1.0 }
    }

    pub fn to_string(&self) -> String {
        String::new()
            + "r: "
            + &self.r.to_string()
            + "g: "
            + &self.g.to_string()
            + "b: "
            + &self.b.to_string()
            + "a: "
            + &self.a.to_string()
    }

    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.8,
        a: 1.0,
    };
    pub const LIGHT_BLUE: Color = Color {
        r: 0.0,
        g: 0.7,
        b: 1.0,
        a: 1.0,
    };
    pub const YELLOW: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const ORANGE: Color = Color {
        r: 1.0,
        g: 0.6,
        b: 0.0,
        a: 1.0,
    };
    pub const PURPLE: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const TURQUOISE: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const GRAY: Color = Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };
    pub const BROWN: Color = Color {
        r: 0.7,
        g: 0.4,
        b: 0.2,
        a: 1.0,
    };
    pub const LIGHT_GRAY: Color = Color {
        r: 0.8,
        g: 0.8,
        b: 0.8,
        a: 1.0,
    };
    pub const CLEAR: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    pub const DEFAULT: Color = Color::CLEAR;

    pub const ALL: [Color; 14] = [
        Color::BLACK,
        Color::WHITE,
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::LIGHT_BLUE,
        Color::YELLOW,
        Color::ORANGE,
        Color::PURPLE,
        Color::TURQUOISE,
        Color::GRAY,
        Color::BROWN,
        Color::LIGHT_GRAY,
        Color::CLEAR,
    ];

    pub fn random() -> &'static Color {
        use rand::Rng;
        &Color::ALL[rand::thread_rng().gen_range(0..Color::ALL.len())]
    }
}
