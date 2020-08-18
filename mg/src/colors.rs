use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Color {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
    pub alpha: usize
}
