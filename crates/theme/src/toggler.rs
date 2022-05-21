use super::Theme;
use iced_native::widget::toggler::Style;
use iced_native::Color;

pub use iced_native::widget::toggler::StyleSheet;

pub struct Toggler(pub Theme);

impl StyleSheet for Toggler {
    fn active(&self, is_active: bool) -> Style {
        Style {
            background: if is_active {
                self.0.active
            } else {
                self.0.background_secondary
            },
            background_border: None,
            foreground: if is_active {
                Color::WHITE
            } else {
                self.0.active
            },
            foreground_border: None,
        }
    }

    fn hovered(&self, is_active: bool) -> Style {
        Style {
            background: if is_active {
                self.0.active
            } else {
                self.0.background_secondary
            },
            background_border: None,
            foreground: if is_active {
                Color {
                    a: 0.5,
                    ..Color::WHITE
                }
            } else {
                Color {
                    a: 0.5,
                    ..self.0.active
                }
            },
            foreground_border: None,
        }
    }
}
