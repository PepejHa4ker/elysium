use super::Theme;
use iced_native::widget::container::Style;
use iced_native::Color;

pub use iced_native::widget::container::StyleSheet;

pub struct Container(pub Theme);

impl StyleSheet for Container {
    fn style(&self) -> Style {
        Style {
            background: self.0.background.into(),
            text_color: Color::WHITE.into(),
            ..Style::default()
        }
    }
}

pub struct Overlay(pub Theme);

impl StyleSheet for Overlay {
    fn style(&self) -> Style {
        Style {
            background: self.0.overlay.into(),
            ..Style::default()
        }
    }
}
