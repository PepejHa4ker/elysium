use super::Theme;
use iced_native::widget::text_input::Style;
use iced_native::Color;

pub use iced_native::widget::text_input::StyleSheet;

pub struct TextInput(pub Theme);

impl StyleSheet for TextInput {
    fn active(&self) -> Style {
        Style {
            background: self.0.background_secondary.into(),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }

    fn focused(&self) -> Style {
        Style {
            border_width: 1.0,
            border_color: self.0.accent,
            ..self.active()
        }
    }

    fn hovered(&self) -> Style {
        Style {
            border_width: 1.0,
            border_color: Color {
                a: 0.3,
                ..self.0.accent
            },
            ..self.focused()
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb(0.4, 0.4, 0.4)
    }

    fn value_color(&self) -> Color {
        Color::WHITE
    }

    fn selection_color(&self) -> Color {
        self.0.active
    }
}
