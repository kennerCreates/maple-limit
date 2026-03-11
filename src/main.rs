mod app;
mod canvas;
mod document;
mod export;
mod grid;
mod palette;
mod shape;
mod tool;
mod ui;
mod viewport;

use app::App;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Maple Limit - SVG Editor")
        .theme(App::theme)
        .subscription(App::subscription)
        .window_size((1200.0, 800.0))
        .run()
}
