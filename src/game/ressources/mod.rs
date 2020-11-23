use bevy::prelude::*;

// #[derive(Copy, Debug)]
pub struct MyAssets {
    pub fruit_color: Handle<ColorMaterial>,
    pub snake_color: Handle<ColorMaterial>,
    pub tail_color:  Handle<ColorMaterial>,
    pub debug_color: Handle<ColorMaterial>,
}

pub struct Scoreboard {
    pub score: usize,
}

pub struct FpsCounter {
    pub fps: usize,
}
