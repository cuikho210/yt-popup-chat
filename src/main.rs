mod app;

use app::{App, AppMessage};
use iced::{Font, Size, Task, daemon::Appearance, futures::executor::block_on};

pub fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_file(true)
        .with_line_number(true)
        .init();

    iced::application("Popup Chat", App::update, App::view)
        .centered()
        .decorations(false)
        .transparent(true)
        .window_size(Size {
            width: 400.,
            height: 400.,
        })
        .style(|_state, _theme| Appearance {
            background_color: iced::Color::from_linear_rgba(0., 0., 0., 0.3),
            text_color: iced::Color::WHITE,
        })
        .default_font(Font::with_name("Noto Sans"))
        .run_with(|| {
            let args: Vec<String> = std::env::args().collect();
            let url = args
                .get(1)
                .expect("Please provide a YouTube video URL as the first command-line argument.");

            let app = block_on(App::try_new(url)).expect("Failed to initialize App");
            let task = Task::future(async { AppMessage::Tick });
            (app, task)
        })
}
