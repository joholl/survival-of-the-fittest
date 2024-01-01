use crate::game::model::Model;
use game::{actor::Actor, utils::Pos};
use nannou::prelude::*;

mod game {
    pub mod actor;
    pub mod controller;
    pub mod model;
    pub mod utils;
}

fn main() {
    nannou::app(model).update(update).exit(exit).run();
}

pub struct UIModel {
    _window: window::Id,
    game_model: Model,
}

fn model(app: &App) -> UIModel {
    let _window = app.new_window().view(view).build().unwrap();

    // TODO
    let mut game_model = Model::new(500, 500);
    game_model.actors.insert(
        Actor::new(),
        Pos {
            x: 100.0,
            y: 100.0,
            angle: 0.1,
        },
    );
    game_model.actors.insert(
        Actor::new(),
        Pos {
            x: 150.0,
            y: 150.0,
            angle: 0.2,
        },
    );
    game_model.actors.insert(
        Actor::new(),
        Pos {
            x: 200.0,
            y: 200.0,
            angle: 0.3,
        },
    );

    UIModel {
        _window,
        game_model,
    }
}

fn update(_app: &App, _model: &mut UIModel, _update: Update) {}

fn view(app: &App, model: &UIModel, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let wh = pt2(100.0, 100.0);

    for (_actor, actor_pos) in model.game_model.actors.iter() {
        // TODO use glam Vec2 for pos
        let xy = pt2(actor_pos.x, actor_pos.y);
        draw.tri()
            .color(GREEN)
            .wh(wh)
            .xy(xy)
            .rotate(actor_pos.angle);
        dbg!(xy);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn exit(_app: &App, _model: UIModel) {
    // TODO this should not be necessary
    std::process::exit(0);
}
