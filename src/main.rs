use nannou::prelude::*;

mod game {
    mod actor;
    pub mod controller;
    mod model;
    mod utils;
}
struct Model {}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
    // game::controller::start();
}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(_app: &App, _model: &Model, _frame: Frame) {}
