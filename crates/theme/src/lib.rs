// color_from_hex32
#![feature(const_fn_floating_point_arithmetic)]

use iced_native::Color;

pub use container::{Crosshair, Overlay, Transparent};

mod button;
mod checkbox;
mod container;
mod progress_bar;
mod radio;
mod rule;
mod scrollable;
mod slider;
mod text_input;
mod toggler;

pub const fn color_from_hex32(color: u32) -> Color {
    let [b, g, r, a] = color.to_ne_bytes();

    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: a as f32 / 255.0,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    background: Color,
    background_secondary: Color,
    accent: Color,
    active: Color,
    hovered: Color,
    overlay: Color,
}

impl Theme {
    pub const DEFAULT: Self = Self {
        background: color_from_hex32(0xFF_14_16_1B),
        background_secondary: color_from_hex32(0xFF_1A_1D_23),
        accent: color_from_hex32(0xFF_F5_29_01),
        active: color_from_hex32(0xFF_F5_29_01),
        hovered: color_from_hex32(0xFF_80_01_01),
        overlay: color_from_hex32(0x7F_14_16_1B),
    };
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::DEFAULT
    }
}

impl<'a> From<Theme> for Box<dyn button::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        button::Button(theme).into()
    }
}

impl<'a> From<Theme> for Box<dyn checkbox::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        checkbox::Checkbox(theme).into()
    }
}

impl<'a> From<Theme> for Box<dyn container::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        container::Container(theme).into()
    }
}

impl From<Theme> for Box<dyn progress_bar::StyleSheet> {
    fn from(theme: Theme) -> Self {
        progress_bar::ProgressBar(theme).into()
    }
}

impl<'a> From<Theme> for Box<dyn radio::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        radio::Radio(theme).into()
    }
}

impl From<Theme> for Box<dyn rule::StyleSheet> {
    fn from(theme: Theme) -> Self {
        rule::Rule(theme).into()
    }
}

impl<'a> From<Theme> for Box<dyn scrollable::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        scrollable::Scrollable(theme).into()
    }
}

impl<'a> From<Theme> for Box<dyn slider::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        slider::Slider(theme).into()
    }
}

impl<'a> From<Theme> for Box<dyn text_input::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        text_input::TextInput(theme).into()
    }
}

impl From<Theme> for Box<dyn toggler::StyleSheet> {
    fn from(theme: Theme) -> Self {
        toggler::Toggler(theme).into()
    }
}
