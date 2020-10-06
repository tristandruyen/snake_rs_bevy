// #![allow(unused_imports)]
// #![allow(unused_parens)]
#![allow(dead_code)]

use bevy::{
    asset::HandleId,
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

use bevy::window::WindowMode;

use crate::plugins::*;

mod prelude;
use prelude::{Direction, *};

mod systems;
use systems::{*, ui::scoreboard_system};

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
                .add_startup_system(setup.system())
                .add_system(scoreboard_system.system())
                .add_system(direction_input_system.system())
                .add_system(bump_snake_tail_system.system())
                .add_system(snake_movement_system.system())
                .add_system(eat_fruit_system.system())
                .run();
}

fn setup(mut commands: Commands,
         mut materials: ResMut<Assets<ColorMaterial>>,
         asset_server: Res<AssetServer>) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
    // cameras

    // score
    .spawn(TextComponents {
        text: Text {
            font: asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap(),
            value: "Score:".to_string(),
            style: TextStyle {
                color: Color::rgb(0.2, 0.2, 0.8),
                font_size: 40.0,
            },
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

    let my_assets =
        MyAssets { fruit_color: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
                   snake_color: materials.add(Color::rgb(0.2, 0.8, 0.2).into()),
                   tail_color:  materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
                   debug_color: materials.add(Color::rgb(1.0, 0.8, 0.2).into()), };

    // snake
    commands
        .spawn(SpriteComponents {
            material: my_assets.snake_color,
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 1.0)),
            sprite: Sprite::new(Vec2::new(15.0, 15.0)),
            ..Default::default()
        })
            .with(Snake{speed: 400.0, direction: Direction::Right});

    // first fruit
    commands
        .spawn(SpriteComponents {
            material: my_assets.fruit_color,
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 1.0)),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(Fruit{ powerup: None});

    commands.insert_resource(my_assets);
}
