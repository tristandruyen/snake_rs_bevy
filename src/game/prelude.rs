use bevy::prelude::*;
pub struct MyAssets {
    pub fruit_color: Handle<ColorMaterial>,
    pub snake_color: Handle<ColorMaterial>,
    pub tail_color:  Handle<ColorMaterial>,
    pub debug_color: Handle<ColorMaterial>,
}
pub struct Scoreboard {
    pub score: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_vec3(&self) -> Vec3 {
        match self {
            Direction::Up => Vec3::new(0.0, 1.0, 0.0),
            Direction::Down => Vec3::new(0.0, -1.0, 0.0),
            Direction::Left => Vec3::new(-1.0, 0.0, 0.0),
            Direction::Right => Vec3::new(1.0, 0.0, 0.0),
        }
    }
}
// struct SnakeHead {}
