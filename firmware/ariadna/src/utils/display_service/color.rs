#[cfg(target_arch = "xtensa")]
use embedded_graphics::pixelcolor::Rgb565;
#[cfg(target_arch = "xtensa")]
use embedded_graphics::prelude::*;

#[derive(Debug)]
pub enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
}

impl Color {
    #[cfg(target_arch = "xtensa")]
    pub fn to_rgb565(&self) -> Rgb565 {
        match self {
            Color::Black => Rgb565::BLACK,
            Color::White => Rgb565::WHITE,
            Color::Red => Rgb565::RED,
            Color::Green => Rgb565::GREEN,
            Color::Blue => Rgb565::BLUE,
        }
    }
}
