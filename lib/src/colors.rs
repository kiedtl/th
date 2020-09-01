use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Color {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
    pub alpha: usize
}

impl From<u32> for Color {
    fn from(u: u32) -> Color {
        Color {
            red:   ((u)       & 0xff) as usize,
            green: ((u >> 8)  & 0xff) as usize,
            blue:  ((u >> 16) & 0xff) as usize,
            alpha: 0,
        }
    }
}

impl Color {
    pub fn new(red: usize, green: usize, blue: usize, alpha: usize) -> Color {
        Color {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
        }
    }

    pub fn as_u32(&self) -> u32 {
        let mut rgb = self.red;
        rgb = (rgb << 8) + self.green;
        rgb = (rgb << 8) + self.blue;
        rgb as u32
    }

    pub fn darken(&self, by: usize) -> Color {
        Color {
            red: self.red / by,
            green: self.green / by,
            blue: self.blue / by,
            alpha: self.alpha,
        }
    }
}
