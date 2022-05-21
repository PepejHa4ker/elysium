use super::Theme;
use iced_native::widget::scrollable::style::{Scrollbar, Scroller};
use iced_native::Color;

pub use iced_native::widget::scrollable::StyleSheet;

pub struct Scrollable(pub Theme);

impl StyleSheet for Scrollable {
    fn active(&self) -> Scrollbar {
        Scrollbar {
            background: self.0.background_secondary.into(),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: Scroller {
                color: self.0.active,
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> Scrollbar {
        let active = self.active();

        Scrollbar {
            background: Color {
                a: 0.5,
                ..self.0.background_secondary
            }
            .into(),
            scroller: Scroller {
                color: self.0.hovered,
                ..active.scroller
            },
            ..active
        }
    }

    fn dragging(&self) -> Scrollbar {
        let hovered = self.hovered();

        Scrollbar {
            scroller: Scroller {
                color: Color::from_rgb(0.85, 0.85, 0.85),
                ..hovered.scroller
            },
            ..hovered
        }
    }
}
