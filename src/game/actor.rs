use std::sync::atomic::{AtomicUsize, Ordering};

use super::utils::Dist;
use super::utils::DistToWalls;
use super::utils::Move;

static COUNTER: AtomicUsize = AtomicUsize::new(1);

// Every actor has an ID to make them distinguishable (to enable using them as
// keys in a HashMap)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Actor {
    _id: usize,
}

impl Actor {
    pub fn new() -> Actor {
        Actor {
            _id: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    fn get_move(&self, nearby_actors: Vec<Dist>, walls: DistToWalls) -> Move {
        Move {
            angle_change: 0.0,
            speed: 0.0,
        }
    }
}
