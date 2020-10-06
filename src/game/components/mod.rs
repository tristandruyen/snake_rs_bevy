use super::prelude::Direction;
use bevy::prelude::Entity;

pub struct Snake {
    pub speed:     f32,
    pub direction: Direction,
}

pub struct SnakeTail {
    pub next_elem: Entity,
    pub direction: Direction,
}

pub struct Fruit {
    pub powerup: Option<PowerUp>,
}

pub enum PowerUp {
    SpeedUp,
}

pub struct Bumper {
    pub direction: Direction,
}
