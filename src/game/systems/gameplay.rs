use bevy::{prelude::*, sprite::collide_aabb::collide};

use rand::Rng;

use crate::game::{components::*, ressources::*};

pub fn tail_catch_system(mut _commands: Commands,
                         mut snake_query: Query<(&Snake, &mut Transform)>,
                         mut snake_tail_query: Query<(&mut SnakeTail,
                                &mut Transform)>) {
    for (snake, mut transform) in &mut snake_query.iter() {
        let snake_pos: Vec4 = transform.value_mut().w_axis();

        for (mut snake_tail, mut transform) in &mut snake_tail_query.iter() {
            let snake_tail_pos: Vec4 = transform.value_mut().w_axis();
            let catching_radius = snake.catching_radius;

            if (snake_tail_pos.x() <= snake_pos.x()
                && snake_tail_pos.x() >= snake_pos.x() - catching_radius / 2.0
                || snake_tail_pos.x() >= snake_pos.x()
                   && snake_tail_pos.x()
                      < snake_pos.x() + catching_radius / 2.0)
               && (snake_tail_pos.y() <= snake_pos.y()
                   && snake_tail_pos.y()
                      >= snake_pos.y() - catching_radius / 2.0
                   || snake_tail_pos.y() >= snake_pos.y()
                      && snake_tail_pos.y()
                         <= snake_pos.y() + catching_radius / 2.0)
            {
                snake_tail.direction = snake.direction;
            }
        }
    }
}

pub fn border_teleport_system(mut _commands: Commands,
                              window_descriptor: Res<WindowDescriptor>,
                              mut query: Query<(&Teleportable,
                                     &mut Transform)>) {
    for (_, mut transform) in &mut query.iter() {
        let width = window_descriptor.width as i32;
        let height = window_descriptor.height as i32;
        let pos: Vec4 = transform.value().w_axis();

        match pos.x() as i32 {
            x if (x > width / 2) => {
                transform.value_mut().w_axis_mut().set_x((x - width) as f32)
            }
            x if (x < -width / 2) => {
                transform.value_mut().w_axis_mut().set_x((x + width) as f32)
            }
            _ => (),
        }

        match pos.y() as i32 {
            y if (y > height / 2) => transform.value_mut()
                                              .w_axis_mut()
                                              .set_y((y - height) as f32),
            y if (y < -height / 2) => transform.value_mut()
                                               .w_axis_mut()
                                               .set_y((y + height) as f32),
            _ => (),
        }
    }
}

pub fn bump_snake_tail_system(mut _commands: Commands,

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

pub fn snake_movement_system(time: Res<Time>,
                             mut snake_query: Query<(&Snake,
                                    &mut Transform)>,
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

pub fn eat_fruit_system(mut commands: Commands,
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
                    .with(SnakeTail{ next_elem: snake_ent, direction: snake.direction})
                    .with(Teleportable);

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
