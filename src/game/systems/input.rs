use bevy::prelude::*;

use crate::game::{prelude::Direction, Bumper, MyAssets, Snake};

pub fn direction_input_system(mut commands: Commands,
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
