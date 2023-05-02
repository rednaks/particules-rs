use crate::common::{self, *};
use macroquad::{
    color::{hsl_to_rgb, rgb_to_hsl},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct Particule {
    pub size: Option<Size>,
    pub color: Option<Color>,
    pub ptype: ParticuleType,
}

impl Particule {
    pub fn draw(&self, position: Position) {
        match self.ptype {
            ParticuleType::EMPTY => {
                //
            }
            _ => draw_rectangle(
                position.x,
                position.y,
                self.size.as_ref().unwrap().width,
                self.size.as_ref().unwrap().height,
                self.color.unwrap(),
            ),
        }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum ParticuleType {
    EMPTY,
    SAND,
    WATER,
    OIL,
    POINTER,
}

impl ParticuleType {
    pub fn get_base_color(self) -> Option<Color> {
        match self {
            Self::SAND => Some(ORANGE),
            Self::WATER => Some(BLUE),
            Self::OIL => Some(GREEN),
            Self::POINTER => Some(RED),
            _ => None,
        }
    }

    pub fn get_random_color_varian(self) -> Option<Color> {
        match self.get_base_color() {
            None => return None,
            Some(color) => {
                let (h, mut s, mut l) = rgb_to_hsl(color);

                s += rand::gen_range(-0.1, 0.1);
                s = common::fit(s, 0.0, 100.0);
                l += rand::gen_range(-0.1, 0.1);
                l = common::fit(l, 0.0, 100.0);

                Some(hsl_to_rgb(h, s, l))
            }
        }
    }

}
