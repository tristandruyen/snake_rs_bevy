#![allow(dead_code)]

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::pass::ClearColor,
};

use bevy::window::WindowMode;

use crate::plugins::*;

mod prelude;
use prelude::{Direction, *};

mod systems;
use systems::*;

pub fn run() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Bevy Borderless Bug Repro".to_owned(),
            width: 800,
            height: 600,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            //mode: WindowMode::Fullscreen { use_size: true }
            // mode: WindowMode::BorderlessFullscreen
            ..Default::default()
        })
        .add_default_plugins()

        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(DebugDiagnosticsPlugin::default())
                .add_resource(Scoreboard { score: 0 })
                .add_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
                .add_startup_system(setup_system.system())
                .add_system(scoreboard_system.system())
                .add_system(direction_input_system.system())
                .add_system(bump_snake_tail_system.system())
                .add_system(snake_movement_system.system())
                .add_system(eat_fruit_system.system())
                .run();
}
