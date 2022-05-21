use super::Theme;
use iced_native::widget::rule::{FillMode, Style};

pub use iced_native::widget::rule::StyleSheet;

pub struct Rule(pub Theme);

impl StyleSheet for Rule {
    fn style(&self) -> Style {
        Style {
            color: self.0.background_secondary,
            width: 2,
            radius: 1.0,
            fill_mode: FillMode::Padded(15),
        }
    }
}
