/// This crate provides us an abstraction for a color type 
/// where the range of [red,blue,green] is from [0.0] to [1.0].
///
#[derive(Clone, Copy, Debug)]
pub struct Color {
    ///Value from 0.0 from 1.0
    pub red: f64,
    ///Value from 0.0 from 1.0
    pub green: f64,
    ///Value from 0.0 from 1.0
    pub blue: f64,
}

macro_rules! const_color {
    ($($id:ident, ($r:expr, $g:expr, $b:expr)),*) => {
       $( pub const $id: Color = Color {
            red: $r,
            green: $g,
            blue: $b,
        };)*
    };
}

const_color! {
    BLACK, (0.0,0.0,0.0) ,
    WHITE, (1.0,1.0,1.0),
    RED, (1.0,0.0,0.0),
    GREEN, (0.0,1.0,0.0),
    BLUE, (1.0,0.0,0.0),
    ORANGE, (1.0,0.65,0.0),
    CYAN, (0.3,0.35,0.35),
    PURPLE,(0.57,0.11,0.7)
}

impl Color {
    ///Create a new Color from values in 0.0 - 1.0. This provides 
    ///a more concise way to create Color values instead of using 
    ///the manual Color {...} style.
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
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
