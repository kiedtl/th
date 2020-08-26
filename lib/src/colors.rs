use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Color {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
    pub alpha: usize
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
}
