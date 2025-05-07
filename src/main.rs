mod app;

use app::{App, AppMessage};
use clap::Parser;
use iced::{Font, Size, Task, daemon::Appearance, futures::executor::block_on};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// YouTube video URL
    #[arg(required = true)]
    url: String,

    /// Background opacity (0.0-1.0)
    #[arg(short, long, default_value = "0.25")]
    opacity: f32,
}

pub fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_file(true)
        .with_line_number(true)
        .init();

    let args = Args::parse();

    iced::application("Popup Chat", App::update, App::view)
        .decorations(false)
        .transparent(true)
        .window_size(Size {
            width: 400.,
            height: 400.,
        })
        .style(move |_state, _theme| Appearance {
            background_color: iced::Color::from_linear_rgba(0., 0., 0., args.opacity),
            text_color: iced::Color::WHITE,
        })
        .default_font(Font::with_name("Noto Sans"))
        .run_with(move || {
            let app = block_on(App::try_new(args.url)).expect("Failed to initialize App");
            let task = Task::future(async { AppMessage::Tick });
            (app, task)
        })
}
