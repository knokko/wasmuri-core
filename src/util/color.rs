#[derive(Clone,Copy,PartialEq,Eq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}

impl Color {

    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha: 255
        }
    }

    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
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

fn u8_to_f32(value: u8) -> f32 {
    value as f32 / 255.0
}