mod app;
mod canvas;
mod document;
mod export;
mod grid;
mod palette;
mod shape;
mod theme;
mod tool;
mod ui;
mod viewport;

use app::App;

const COURIER_PRIME: iced::Font = iced::Font {
    family: iced::font::Family::Name("Courier Prime"),
    weight: iced::font::Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Maple Limit - SVG Editor")
        .theme(App::theme)
        .subscription(App::subscription)
        .font(include_bytes!("../assets/fonts/Courier_Prime/CourierPrime-Regular.ttf"))
        .font(include_bytes!("../assets/fonts/Courier_Prime/CourierPrime-Bold.ttf"))
        .default_font(COURIER_PRIME)
        .window_size((1200.0, 800.0))
        .run()
}
