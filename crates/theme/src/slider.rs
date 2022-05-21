use super::Theme;
use iced_native::widget::slider::{Handle, HandleShape, Style};
use iced_native::Color;

pub use iced_native::widget::slider::StyleSheet;

pub struct Slider(pub Theme);

impl StyleSheet for Slider {
    fn active(&self) -> Style {
        Style {
            rail_colors: (
                self.0.active,
                Color {
                    a: 0.1,
                    ..self.0.active
                },
            ),
            handle: Handle {
                shape: HandleShape::Circle { radius: 9.0 },
                color: self.0.active,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> Style {
        let active = self.active();

        Style {
            handle: Handle {
                color: self.0.hovered,
                ..active.handle
            },
            ..active
        }
    }

    fn dragging(&self) -> Style {
        let active = self.active();

        Style {
            handle: Handle {
                color: Color::from_rgb(0.85, 0.85, 0.85),
                ..active.handle
            },
            ..active
        }
    }
}
