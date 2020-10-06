use bevy::{
    asset::HandleId,
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

use crate::game::prelude::*;

pub fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    for mut text in &mut query.iter() {
        text.value = format!("Score: {}", scoreboard.score);
    }
}
