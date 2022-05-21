use super::Theme;
use iced_native::widget::progress_bar::Style;

pub use iced_native::widget::progress_bar::StyleSheet;

pub struct ProgressBar(pub Theme);

impl StyleSheet for ProgressBar {
    fn style(&self) -> Style {
        Style {
            background: self.0.background_secondary.into(),
            bar: self.0.active.into(),
            border_radius: 10.0,
        }
    }
}
