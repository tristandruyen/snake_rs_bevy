use bevy::prelude::*;

use crate::game::{components::*, prelude::Direction, ressources::*};

pub fn setup_system(mut commands: Commands,
                    mut materials: ResMut<Assets<ColorMaterial>>,
                    asset_server: Res<AssetServer>) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
    // cameras

    // score
    .spawn(TextComponents {
        text: Text {
            font: asset_server.load("/home/tristand/code/snake_rs_bevy/assets/fonts/FiraSans-Bold.ttf"),
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
    }).with(ScoreText{})
    // fps counter
    .spawn(TextComponents {
        text: Text {
            font: asset_server.load("/home/tristand/code/snake_rs_bevy/assets/fonts/FiraSans-Bold.ttf"),
            value: "FPS:".to_string(),
            style: TextStyle {
                color: Color::rgb(0.2, 0.2, 0.8),
                font_size: 40.0,
            },
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(500.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }).with(FpsText);

    let my_assets =
        MyAssets { fruit_color: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
                   snake_color: materials.add(Color::rgb(0.2, 0.8, 0.2).into()),
                   tail_color:  materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
                   debug_color: materials.add(Color::rgb(1.0, 0.8, 0.2).into()), };

    // snake
    commands
        .spawn(SpriteComponents {
            material: my_assets.snake_color.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 1.0)),
            sprite: Sprite::new(Vec2::new(15.0, 15.0)),
            ..Default::default()
        })
        .with(Snake{speed: 400.0, direction: Direction::Right})
        .with(Teleportable)
        .with(Catcher{catching_radius: 20.0});

    // first fruit
    commands
        .spawn(SpriteComponents {
            material: my_assets.fruit_color.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 1.0)),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(Fruit{ powerup: None});

    commands.insert_resource(my_assets);
}
