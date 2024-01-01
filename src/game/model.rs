// Keeps track of where all the actors are
// Which actors get eaten?
// Update all actors after move
use super::actor::Actor;
use super::utils::Pos;
use std::collections::HashMap;

static ACTOR_FOV: u8 = 0;

struct Board {
    width: usize,
    height: usize,
}

pub struct Model {
    board: Board,
    actors: HashMap<Actor, Pos>,
    actor_fov: u8,
}

impl Model {
    pub fn new(width: usize, height: usize) -> Model {
        let actors: HashMap<Actor, Pos> = HashMap::new(); // TODO
        Model {
            board: Board { width, height },
            actors,
            actor_fov: ACTOR_FOV,
        }
    }

    // Call on each tick
    pub fn update() {
        // TODO: this whole method
        // For each actor, pick out the closest ones (restricted by field of view!!), calculate the relative distance

        // Evaluate which actors are gonna eat eachother, at the end of movement?
    }
}
