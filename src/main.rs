use crate::components::aero_button::aero_button;
use iced::Length;
use iced::widget::column;
use iced::widget::row;
use iced::{Alignment, Task as Command, Theme};
use iced_widget::text;

mod components;

fn main() -> iced::Result {
    iced::application("iced_aero demo", App::update, App::view)
        .theme(|_| Theme::Light)
        .run_with(App::new)
}

#[derive(Default)]
struct App {}

impl App {
    fn new() -> (Self, Command<Message>) {
        (Self {}, Command::none())
    }

    fn view(&'_ self) -> iced::Element<'_, Message> {
        let mut font = iced::Font::default();
        font.weight = iced::font::Weight::Medium;

        row![
            column![
                aero_button(column![text("Button").font(font).size(15)].align_x(Alignment::Center))
                    .on_press(Message::ButtonPressed)
            ]
            .width(Length::Fill)
            .align_x(Alignment::Center)
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_y(Alignment::Center)
        .into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ButtonPressed => {
                println!("Button Pressed")
            }
        }
        Command::none()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed,
}
