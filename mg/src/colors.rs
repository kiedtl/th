use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Color {
    red: usize,
    green: usize,
    blue: usize,
    alpha: usize
}
