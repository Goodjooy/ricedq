use iced::{Application, Font, Settings};

mod app;
mod setting;
mod weights;
fn main() {
    println!("Hello, world!");
    app::App::run(Settings::with_flags(())).expect("error");
}

const FONT: Font = Font::External {
    name: "黑体 常规",
    bytes: include_bytes!("../resource/simhei.ttf"),
};
