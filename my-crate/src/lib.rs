//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::RGBColor;
pub use self::kinds::Shape;
pub use self::kinds::Style;
pub use self::utils::mix;


pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum RGBColor {
        Red,
        Green,
        Blue,
    }

    /// Shapes that can be drawn.
    pub enum Shape {
        Circle,
        Square,
        Triangle,
    }

    /// Styles that can be applied to a shape.
    pub enum Style {
        Solid,
        Dotted,
        Dashed,
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: RGBColor, c2: RGBColor) -> RGBColor {
        match (c1, c2) {
            (RGBColor::Red, RGBColor::Green) | (RGBColor::Green, RGBColor::Red) => RGBColor::Blue,
            (RGBColor::Red, RGBColor::Blue) | (RGBColor::Blue, RGBColor::Red) => RGBColor::Green,
            (RGBColor::Green, RGBColor::Blue) | (RGBColor::Blue, RGBColor::Green) => RGBColor::Red,
            (RGBColor::Red, RGBColor::Red) => RGBColor::Red,
            (RGBColor::Green, RGBColor::Green) => RGBColor::Green,
            (RGBColor::Blue, RGBColor::Blue) => RGBColor::Blue,
        }
    }
}
