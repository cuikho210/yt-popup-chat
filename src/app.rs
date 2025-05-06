use iced::Center;
use iced::widget::{Column, button, column, text};

#[derive(Debug, Clone, Copy)]
pub enum AppMessage {
    Increment,
    Decrement,
}

#[derive(Default)]
pub struct App {
    value: i64,
}
impl App {
    pub fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::Increment => {
                self.value += 1;
            }
            AppMessage::Decrement => {
                self.value -= 1;
            }
        }
    }

    pub fn view(&self) -> Column<AppMessage> {
        column![
            button("Increment").on_press(AppMessage::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(AppMessage::Decrement)
        ]
        .padding(20)
        .align_x(Center)
    }
}
