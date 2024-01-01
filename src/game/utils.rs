pub struct Dist {
    pub relative_angle: f32,
    pub distance: f32,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

pub struct DistToWalls {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

pub struct Move {
    pub angle_change: f32,
    pub speed: f32,
}
