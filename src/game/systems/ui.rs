use bevy::{
    core::Time,
    diagnostic::Diagnostics,
    ecs::{Res, ResMut},
    prelude::*,
};

use crate::game::{
    components::{FpsText, ScoreText},
    ressources::*,
};

use crate::plugins::debug_diagnostics_plugin::DebugDiagnosticsState;

pub fn scoreboard_system(scoreboard: Res<Scoreboard>,
                         mut query: Query<(&mut Text, &ScoreText)>) {
    for (mut text, _) in query.iter_mut() {
        text.value = format!("Score: {}", scoreboard.score);
    }
}

pub fn fps_counter_system(mut state: ResMut<DebugDiagnosticsState>,
                          time: Res<Time>,
                          diagnostics: Res<Diagnostics>,
                          mut query: Query<(&mut Text, &FpsText)>) {
    state.timer.tick(time.delta_seconds);

    if state.timer.finished {
        if let Some(ref _filter) = state.filter {
        } else {
            for diagnostic in diagnostics.iter() {
                if diagnostic.name != "fps" {
                    continue;
                }

                for (mut text, _) in query.iter_mut() {
                    text.value =
                        format!("Fps: {:?}", diagnostic.value().unwrap());
                }
            }
        }
    }
}
