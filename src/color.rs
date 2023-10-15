#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn red() -> Color {
        Color {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn green() -> Color {
        Color {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
        }
    }

    pub fn blue() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
        }
    }

    pub fn black() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn white() -> Color {
        Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }

    pub fn color_into_pixel(&self) -> (u8, u8, u8) {
        let red = Self::scale_color_component(self.red);
        let blue = Self::scale_color_component(self.blue);
        let green = Self::scale_color_component(self.green);
        (red, green, blue)
    }

    fn scale_color_component(component: f64) -> u8 {
        let component = if component < 0.0 {
            0.0
        } else if component > 1.0 {
            1.0
        } else {
            component
        };

        (component * 255.0) as u8
    }
}
