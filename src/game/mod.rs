#![allow(dead_code)]

use bevy::{
    audio::AudioPlugin,
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::pass::ClearColor,
    DefaultPlugins,
};

use bevy::window::WindowMode;

use crate::plugins::*;

mod prelude;

mod systems;
use systems::*;

mod components;
use components::*;

mod ressources;
use ressources::*;

pub fn run() {
    App::build().add_resource(WindowDescriptor { title: "Snake".to_owned(),
                                                 width: 800,
                                                 height: 600,
                                                 vsync: false,
                                                 resizable: false,
                                                 mode: WindowMode::Windowed,
                                                 //mode: WindowMode::Fullscreen { use_size: true }
                                                 // mode: WindowMode::BorderlessFullscreen
                                                 ..Default::default() })
                .add_plugins_with(DefaultPlugins, |group| {
                    group.disable::<AudioPlugin>()
                })
                .add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(DebugDiagnosticsPlugin::default())
                .add_resource(Scoreboard { score: 0 })
                .add_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
                .add_startup_system(setup_system.system())
                .add_system(scoreboard_system.system())
                .add_system(fps_counter_system.system())
                .add_system(border_teleport_system.system())
                .add_system(direction_input_system.system())
                .add_system(bump_snake_tail_system.system())
                .add_system(snake_movement_system.system())
                .add_system(eat_fruit_system.system())
                .add_system(tail_catch_system.system())
                .run();
}
