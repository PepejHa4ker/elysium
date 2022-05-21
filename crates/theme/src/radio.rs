use super::Theme;
use iced_native::widget::radio::Style;
use iced_native::Color;

pub use iced_native::widget::radio::StyleSheet;

pub struct Radio(pub Theme);

impl StyleSheet for Radio {
    fn active(&self) -> Style {
        Style {
            background: self.0.background_secondary.into(),
            dot_color: self.0.active,
            border_width: 1.0,
            border_color: self.0.active,
            text_color: None,
        }
    }

    fn hovered(&self) -> Style {
        Style {
            background: Color {
                a: 0.5,
                ..self.0.background_secondary
            }
            .into(),
            ..self.active()
        }
    }
}
