use std::{default, path::{Path, PathBuf}};
use iced::{Alignment, Color, Element, Theme, Font, Length::Fill, Task, widget::{button, column, container, row, text}, window};
mod flask;

#[derive(Debug)]
struct AppState{
    current_dir: PathBuf,

}

impl Default for AppState {
    fn default() -> Self {
        AppState { current_dir: std::env::current_dir().unwrap() }
    }
}

impl AppState {
    fn update(&mut self, message: Message) -> Task<Message>{
        match message {
            Message::Exit => window::latest().and_then(window::close),
        }  
    }

 

    fn view(&self) -> Element<Message> {

        let main_style = |theme: &Theme| {
            container::Style {
                background: Some(Color::from_rgb(0.1, 0.1, 0.1).into()),
                ..Default::default()
            }
        };

        container(
            column![
                row![
                    text("Testify")
                    .font(Font::MONOSPACE)
                    .size(60)
                    .line_height(1.75)
                    .width(Fill)
                    .height(Fill)
                    .align_x(Alignment::Center),
                    button(text("Exit")).on_press(Message::Exit)
                ]
            ]
        ).style(main_style).into()
    }

}

#[derive(Debug, Clone)]
enum Message {
    Exit,
}

fn main() -> iced::Result {
    iced::run(AppState::update, AppState::view)
}
