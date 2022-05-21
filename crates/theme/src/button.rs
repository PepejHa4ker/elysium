use super::Theme;
use iced_native::widget::button::Style;
use iced_native::Color;

pub use iced_native::widget::button::StyleSheet;

pub struct Button(pub Theme);

impl StyleSheet for Button {
    fn active(&self) -> Style {
        Style {
            background: self.0.active.into(),
            border_radius: 3.0,
            text_color: self.0.background.into(),
            ..Style::default()
        }
    }

    fn hovered(&self) -> Style {
        Style {
            border_width: 1.0,
            border_color: Color::WHITE,
            ..self.active()
        }
    }

    fn pressed(&self) -> Style {
        Style {
            background: self.0.hovered.into(),
            text_color: self.0.background.into(),
            ..self.hovered()
        }
    }
}
