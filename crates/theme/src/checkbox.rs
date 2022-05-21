use super::Theme;
use iced_native::widget::checkbox::Style;
use iced_native::Color;

pub use iced_native::widget::checkbox::StyleSheet;

pub struct Checkbox(pub Theme);

impl StyleSheet for Checkbox {
    fn active(&self, is_checked: bool) -> Style {
        Style {
            background: if is_checked {
                self.0.active
            } else {
                self.0.background_secondary
            }
            .into(),
            checkmark_color: Color::WHITE,
            border_radius: 2.0,
            border_width: 1.0,
            border_color: self.0.active,
            text_color: None,
        }
    }

    fn hovered(&self, is_checked: bool) -> Style {
        Style {
            background: Color {
                a: 0.8,
                ..if is_checked {
                    self.0.active
                } else {
                    self.0.background_secondary
                }
            }
            .into(),
            ..self.active(is_checked)
        }
    }
}
