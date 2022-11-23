use app::App;
use iced::{Application, Settings};

extern crate area_calc;
extern crate common;
extern crate iced;
extern crate integral_eq;
extern crate mathparse;
extern crate min_find;
extern crate spline;

mod app;
mod views;

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (500, 500);

    App::run(settings)
}
