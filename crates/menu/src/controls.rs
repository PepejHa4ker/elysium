use elysium_theme::Theme;
use iced_elysium_gl::Renderer;
use iced_native::widget::{
    button, scrollable, slider, text_input, Button, Checkbox, Column, Container, ProgressBar, Row,
    Rule, Scrollable, Slider, Space, Text, TextInput, Toggler,
};
use iced_native::{Alignment, Command, Element, Length, Program};

#[derive(Default)]
pub struct Controls {
    theme: Theme,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    button: button::State,
    slider: slider::State,
    slider_value: f32,
    checkbox_value: bool,
    toggler_value: bool,
    menu_visibility: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    ButtonPressed,
    SliderChanged(f32),
    CheckboxToggled(bool),
    TogglerToggled(bool),
    MenuVisibility(bool),
}

impl Controls {
    #[inline]
    pub fn new() -> Controls {
        Default::default()
    }
}

impl Program for Controls {
    type Renderer = Renderer;
    type Message = Message;

    #[inline]
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(value) => self.input_value = value,
            Message::ButtonPressed => {}
            Message::SliderChanged(value) => self.slider_value = value,
            Message::CheckboxToggled(value) => self.checkbox_value = value,
            Message::TogglerToggled(value) => self.toggler_value = value,
            Message::MenuVisibility(value) => self.menu_visibility = value,
        }

        Command::none()
    }

    #[inline]
    fn view(&mut self) -> Element<Message, Renderer> {
        let text_input = TextInput::new(
            &mut self.input,
            "sample text sample text",
            &self.input_value,
            Message::InputChanged,
        )
        .padding(10)
        .size(20)
        .style(self.theme);

        let button = Button::new(&mut self.button, Text::new("inform ccp"))
            .padding(10)
            .on_press(Message::ButtonPressed)
            .style(self.theme);

        let slider = Slider::new(
            &mut self.slider,
            0.0..=100.0,
            self.slider_value,
            Message::SliderChanged,
        )
        .style(self.theme);

        let progress_bar = ProgressBar::new(0.0..=100.0, self.slider_value).style(self.theme);

        let scrollable = Scrollable::new(&mut self.scroll)
            .width(Length::Fill)
            .height(Length::Units(100))
            .style(self.theme)
            .push(Text::new("scroll"))
            .push(Space::with_height(Length::Units(800)))
            .push(Text::new("scrolled"));

        let checkbox = Checkbox::new(self.checkbox_value, "checkbox", Message::CheckboxToggled)
            .style(self.theme);

        let toggler = Toggler::new(
            self.toggler_value,
            String::from("togger"),
            Message::TogglerToggled,
        )
        .width(Length::Shrink)
        .spacing(10)
        .style(self.theme);

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(Rule::horizontal(38).style(self.theme))
            .push(Row::new().spacing(10).push(text_input).push(button))
            .push(slider)
            .push(progress_bar)
            .push(
                Row::new()
                    .spacing(10)
                    .height(Length::Units(100))
                    .align_items(Alignment::Center)
                    .push(scrollable)
                    .push(Rule::vertical(38).style(self.theme))
                    .push(
                        Column::new()
                            .width(Length::Shrink)
                            .spacing(20)
                            .push(checkbox)
                            .push(toggler),
                    ),
            );

        let menu = Container::new(content)
            .width(Length::Units(800))
            .height(Length::Units(640))
            .center_x()
            .center_y()
            .style(self.theme);

        Container::new(menu)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(elysium_theme::Overlay(self.theme))
            .into()
    }
}
