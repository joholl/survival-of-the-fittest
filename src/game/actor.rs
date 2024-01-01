use super::utils::Dist;
use super::utils::DistToWalls;
use super::utils::Move;

pub struct Actor;
impl Actor {
    fn get_move(&self, nearby_actors: Vec<Dist>, walls: DistToWalls) -> Move {
        Move {
            angle_change: 0.0,
            speed: 0.0,
        }
    }
}
