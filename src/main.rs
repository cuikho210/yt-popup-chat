mod app;

use app::App;
use iced::{Size, daemon::Appearance};

pub fn main() -> iced::Result {
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
            text_color: Default::default(),
        })
        .run()
}
