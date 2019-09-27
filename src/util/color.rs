#[derive(Clone,Copy,PartialEq,Eq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}

impl Color {

    pub const BLACK: Color = Color::from_rgb(0, 0, 0);
    pub const WHITE: Color = Color::from_rgb(255, 255, 255);

    pub const RED: Color = Color::from_rgb(255, 0, 0);
    pub const GREEN: Color = Color::from_rgb(0, 255, 0);
    pub const BLUE: Color = Color::from_rgb(0, 0, 255);

    pub const TRANSPARENT: Color = Color::from_rgba(0,0,0,0);

    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha: 255
        }
    }

    pub const fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha
        }
    }

    pub fn get_red_float(&self) -> f32 {
        u8_to_f32(self.red)
    }

    pub fn get_green_float(&self) -> f32 {
        u8_to_f32(self.green)
    }

    pub fn get_blue_float(&self) -> f32 {
        u8_to_f32(self.blue)
    }

    pub fn get_alpha_float(&self) -> f32 {
        u8_to_f32(self.alpha)
    }

    pub fn get_red(&self) -> u8 {
        self.red
    }

    pub fn get_green(&self) -> u8 {
        self.green
    }

    pub fn get_blue(&self) -> u8 {
        self.blue
    }

    pub fn get_alpha(&self) -> u8 {
        self.alpha
    }
}

#[derive(Clone,Copy)]
pub struct TextColors {

    pub fill_color: Color,
    pub stroke_color: Color,
    pub background_color: Color
}

impl TextColors {

    pub const fn new(fill_color: Color, stroke_color: Color, background_color: Color) -> TextColors {
        TextColors {
            fill_color,
            stroke_color,
            background_color
        }
    }

    pub const fn create_label(fill_color: Color, stroke_color: Color) -> TextColors {
        TextColors {
            fill_color,
            stroke_color,
            background_color: Color::TRANSPARENT
        }
    }

    pub const fn create_simple_label(color: Color) -> TextColors {
        Self::create_label(color, color)
    }

    pub const BLACK_LABEL: TextColors = Self::create_simple_label(Color::BLACK);

    pub const fn create_simple_button(background_color: Color) -> TextColors {
        TextColors {
            fill_color: Color::BLACK,
            stroke_color: Color::BLACK,
            background_color
        }
    }
}

fn u8_to_f32(value: u8) -> f32 {
    value as f32 / 255.0
}