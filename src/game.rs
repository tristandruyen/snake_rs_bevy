#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(dead_code)]

use bevy::{
    asset::HandleId,
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

use bevy::window::WindowMode;

use rand::Rng;

struct MyAssets {
    fruit_color: Handle<ColorMaterial>,
    snake_color: Handle<ColorMaterial>,
    tail_color:  Handle<ColorMaterial>,
    debug_color: Handle<ColorMaterial>,
}

struct Scoreboard {
    score: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_vec3(&self) -> Vec3 {
        match self {
            Direction::Up => Vec3::new(0.0, 1.0, 0.0),
            Direction::Down => Vec3::new(0.0, -1.0, 0.0),
            Direction::Left => Vec3::new(-1.0, 0.0, 0.0),
            Direction::Right => Vec3::new(1.0, 0.0, 0.0),
        }
    }
}

struct Snake {
    speed:     f32,
    direction: Direction,
}

enum PowerUp {
    SpeedUp,
}

struct Fruit {
    powerup: Option<PowerUp>,
}

// struct SnakeHead {}

struct SnakeTail {
    next_elem: Entity,
    direction: Direction,
}

struct Bumper {
    direction: Direction,
}

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
        .add_plugin(PrintDiagnosticsPlugin::default())
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

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    for mut text in &mut query.iter() {
        text.value = format!("Score: {}", scoreboard.score);
    }
}

fn change_direction_for_parts_system(mut snake_query: Query<(Entity,
                                            &SnakeTail,
                                            &Transform,
                                            &Sprite)>) {
    for (_snake_tail_entity,
         _snake_tail,
         _snake_tail_transform,
         _snake_tail_sprite) in &mut snake_query.iter()
    {}
}

fn direction_input_system(mut commands: Commands,
                          my_assets: Res<MyAssets>,
                          keyboard_input: Res<Input<KeyCode>>,
                          mut query: Query<(&mut Snake, &Transform)>) {
    for (mut snake, transform) in &mut query.iter() {
        for input in keyboard_input.get_pressed() {
            if let Some(direction) = match &input {
                KeyCode::Left => Some(Direction::Left),

                KeyCode::Right => Some(Direction::Right),
                KeyCode::Down => Some(Direction::Down),
                KeyCode::Up => Some(Direction::Up),
                _ => (None),
            } {
                if direction != snake.direction {
                    snake.direction = direction;

                    commands.spawn(SpriteComponents {
                        material: my_assets.debug_color,
                        transform: *transform,
                        sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                        ..Default::default()
                    })
                    .with(Bumper{ direction});
                }
            }
        }
    }
}

fn bump_snake_tail_system(mut _commands: Commands,

                          mut bumper_query: Query<(Entity,
                                 &Bumper,
                                 &Transform,
                                 &Sprite)>,
                          mut snake_tail_query: Query<(Entity,
                                 &mut SnakeTail,
                                 &Transform,
                                 &Sprite)>) {
    for (_bumper_ent, bumper, bumper_transform, bumper_sprite) in
        &mut bumper_query.iter()
    {
        // let vec_tmp = bumper.direction.to_vec3();
        // let bumper_size = Vec2::new(vec_tmp.x(), vec_tmp.y());
        let bumper_size = bumper_sprite.size;

        for (_snake_tail_ent,
             mut snake_tail,
             snake_tail_transform,
             snake_tail_sprite) in &mut snake_tail_query.iter()
        {
            let collision = collide(bumper_transform.translation(),
                                    bumper_size,
                                    snake_tail_transform.translation(),
                                    snake_tail_sprite.size);
            if collision.is_some() {
                snake_tail.direction = bumper.direction;
                // println!("asd")
            }
        }
    }
}

fn eat_fruit_system(mut commands: Commands,
                    mut scoreboard: ResMut<Scoreboard>,
                    my_assets: Res<MyAssets>,
                    mut snake_query: Query<(Entity,
                           &Snake,
                           &Transform,
                           &Sprite)>,
                    mut fruit_query: Query<(Entity,
                           &Fruit,
                           &Transform,
                           &Sprite)>) {
    for (snake_ent, snake, snake_transform, snake_sprite) in
        &mut snake_query.iter()
    {
        let snake_size = snake_sprite.size;

        for (fruit_entity, mut _fruit, fruit_transform, fruit_sprite) in
            &mut fruit_query.iter()
        {
            let collision = collide(snake_transform.translation(),
                                    snake_size,
                                    fruit_transform.translation(),
                                    fruit_sprite.size);

            if collision.is_some() {
                commands.despawn(fruit_entity);
                scoreboard.score += 1;

                // spawn new snake tail

                let mut part_transform = *snake_transform;
                part_transform.translate(snake.direction.to_vec3()
                                         * Vec3::new(-snake_size.x(),
                                                     -snake_size.y(),
                                                     0.0));

                commands
                    .spawn(SpriteComponents {
                        material: my_assets.tail_color,
                        transform:  part_transform,
                        sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                        ..Default::default()
                    })
                    .with(SnakeTail{ next_elem: snake_ent, direction: snake.direction});

                // spawnm new random fruit

                let mut rng = rand::thread_rng();

                commands
                    .spawn(SpriteComponents {
                        material: my_assets.fruit_color,
                        transform: Transform::from_translation(
                            Vec3::new(
                                rng.gen_range(-250.0, 250.0),
                                rng.gen_range(-250.0, 250.0),
                                1.0
                            )
                        ),
                        sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                        ..Default::default()
                    })
                    .with(Fruit{ powerup: None});
            }
        }
    }
}

fn snake_movement_system(time: Res<Time>,
                         mut snake_query: Query<(&Snake, &mut Transform)>,
                         mut snake_tail_query: Query<(&SnakeTail,
                                &mut Transform)>) {
    // // move the paddle horizontally
    // *translation.x_mut() += time.delta_seconds * direction * paddle.speed;
    // // bound the paddle within the walls
    // *translation.x_mut() = translation.x().min(380.0).max(-380.0);
    // clamp the timestep to stop the ball from escaping when the game starts

    let delta_seconds = f32::min(0.01, time.delta_seconds);

    for (snake, mut transform) in &mut snake_query.iter() {
        transform.translate(snake.direction.to_vec3()
                            * snake.speed
                            * delta_seconds);
    }

    for (snake_tail, mut tail_transform) in &mut snake_tail_query.iter() {
        tail_transform.translate(snake_tail.direction.to_vec3()
                                 * 400.0
                                 * delta_seconds);
    }
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
