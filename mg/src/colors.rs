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
}
