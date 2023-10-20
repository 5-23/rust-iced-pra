use iced::{Sandbox, widget::text, Settings};
use iced::widget::text_input;
fn main() -> iced::Result {
    Editor::run(Settings::default())
}

#[derive(Debug, Clone)]
struct Editor;

#[derive(Debug, Clone)]
enum Message{}

impl Sandbox for Editor{
    type Message = Message;
    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        "rust editor".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message{

        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        text_input("input here", "").into()
    }
}