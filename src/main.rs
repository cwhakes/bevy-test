use bevy::{
	prelude::*,
	render::camera::Camera,
	render::pass::ClearColor,
	render::render_graph::base::camera::CAMERA_2D,
	sprite::collide_aabb::{collide, Collision},
};
use rand::prelude::*;

/// An implementation of the classic game "Breakout"
fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		.insert_resource(Scoreboard { score: 0 })
		.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
		.insert_resource(PlatformTimer(Timer::from_seconds(1.0, true)))
		.add_startup_system(setup.system())
		.add_system(paddle_movement_system.system())
		.add_system(ball_collision_system.system())
		.add_system(ball_movement_system.system())
		.add_system(ball_gravity_system.system())
		.add_system(scoreboard_system.system())
		.add_system(platform_spawner_system.system())
		.add_system(camera_tracking_system.system())
		.run();
}

struct Paddle {
	speed: f32,
}

struct Ball {
	velocity: Vec3,
}

struct Scoreboard {
	score: usize,
}

struct Gravity;

struct Platform;

struct PlatformTimer(Timer);

struct Collider;

fn setup(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	asset_server: Res<AssetServer>,
) {
	// Add the game's entities to our world

	// cameras
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands.spawn_bundle(UiCameraBundle::default());
	// paddle
	// commands
	//     .spawn_bundle(SpriteBundle {
	//         material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
	//         transform: Transform::from_xyz(0.0, -215.0, 0.0),
	//         sprite: Sprite::new(Vec2::new(120.0, 30.0)),
	//         ..Default::default()
	//     })
	//     .insert(Paddle { speed: 500.0 })
	//     .insert(Collider::Paddle);
	// ball
	commands
		.spawn_bundle(SpriteBundle {
			material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
			transform: Transform::from_xyz(0.0, -50.0, 1.0),
			sprite: Sprite::new(Vec2::new(30.0, 30.0)),
			..Default::default()
		})
		.insert(Ball {
			velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
		})
		.insert(Gravity);
	// scoreboard
	commands.spawn_bundle(TextBundle {
		text: Text {
			sections: vec![
				TextSection {
					value: "Score: ".to_string(),
					style: TextStyle {
						font: asset_server.load("fonts/FiraSans-Bold.ttf"),
						font_size: 40.0,
						color: Color::rgb(0.5, 0.5, 1.0),
					},
				},
				TextSection {
					value: "".to_string(),
					style: TextStyle {
						font: asset_server.load("fonts/FiraMono-Medium.ttf"),
						font_size: 40.0,
						color: Color::rgb(1.0, 0.5, 0.5),
					},
				},
			],
			..Default::default()
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

	// Add walls
	// let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
	// let wall_thickness = 10.0;
	// let bounds = Vec2::new(900.0, 600.0);

	// // left
	// commands
	//     .spawn_bundle(SpriteBundle {
	//         material: wall_material.clone(),
	//         transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
	//         sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
	//         ..Default::default()
	//     })
	//     .insert(Collider::Solid);
	// // right
	// commands
	//     .spawn_bundle(SpriteBundle {
	//         material: wall_material.clone(),
	//         transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
	//         sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
	//         ..Default::default()
	//     })
	//     .insert(Collider::Solid);
	// bottom
	// commands
	// 	.spawn_bundle(SpriteBundle {
	// 		material: wall_material.clone(),
	// 		transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
	// 		sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
	// 		..Default::default()
	// 	})
	// 	.insert(Collider::Solid);
	// // top
	// commands
	//     .spawn_bundle(SpriteBundle {
	//         material: wall_material,
	//         transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
	//         sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
	//         ..Default::default()
	//     })
	//     .insert(Collider::Solid);

	// Add bricks
	// let brick_rows = 4;
	// let brick_columns = 5;
	// let brick_spacing = 20.0;
	// let brick_size = Vec2::new(150.0, 30.0);
	// let bricks_width = brick_columns as f32 * (brick_size.x + brick_spacing) - brick_spacing;
	// // center the bricks and move them up a bit
	// let bricks_offset = Vec3::new(-(bricks_width - brick_size.x) / 2.0, 100.0, 0.0);
	// let brick_material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
	// for row in 0..brick_rows {
	//     let y_position = row as f32 * (brick_size.y + brick_spacing);
	//     for column in 0..brick_columns {
	//         let brick_position = Vec3::new(
	//             column as f32 * (brick_size.x + brick_spacing),
	//             y_position,
	//             0.0,
	//         ) + bricks_offset;
	//         // brick
	//         commands
	//             .spawn_bundle(SpriteBundle {
	//                 material: brick_material.clone(),
	//                 sprite: Sprite::new(brick_size),
	//                 transform: Transform::from_translation(brick_position),
	//                 ..Default::default()
	//             })
	//             .insert(Collider::Scorable);
	//     }
	// }
}

fn paddle_movement_system(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&Paddle, &mut Transform)>,
) {
	if let Ok((paddle, mut transform)) = query.single_mut() {
		let mut direction = 0.0;
		if keyboard_input.pressed(KeyCode::Left) {
			direction -= 1.0;
		}

		if keyboard_input.pressed(KeyCode::Right) {
			direction += 1.0;
		}

		let translation = &mut transform.translation;
		// move the paddle horizontally
		translation.x += time.delta_seconds() * direction * paddle.speed;
		// bound the paddle within the walls
		translation.x = translation.x.min(380.0).max(-380.0);
	}
}

fn ball_movement_system(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Transform)>) {
	// clamp the timestep to stop the ball from escaping when the game starts
	let delta_seconds = f32::min(0.2, time.delta_seconds());

	if let Ok((ball, mut transform)) = ball_query.single_mut() {
		transform.translation += ball.velocity * delta_seconds;
	}
}

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
	let mut text = query.single_mut().unwrap();
	text.sections[0].value = format!("Score: {}", scoreboard.score);
}

fn ball_collision_system(
	mut commands: Commands,
	mut scoreboard: ResMut<Scoreboard>,
	mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
	collider_query: Query<(Entity, &Transform, &Sprite), With<Collider>>,
) {
	if let Ok((mut ball, ball_transform, sprite)) = ball_query.single_mut() {
		let ball_size = sprite.size;
		let velocity = &mut ball.velocity;

		// check collision with walls
		for (collider_entity, transform, sprite) in collider_query.iter() {
			let collision = collide(
				ball_transform.translation,
				ball_size,
				transform.translation,
				sprite.size,
			);
			if let Some(collision) = collision {
                scoreboard.score += 1;
                commands.entity(collider_entity).despawn();

				// only reflect if the ball's velocity is going in the opposite direction of the
				// collision
                if let Collision::Top = collision {
                    if velocity.y < 0.0 {
                        velocity.y = -velocity.y;
                    }
                }
			}
		}
	}
}

fn ball_gravity_system(time: Res<Time>, mut ball_query: Query<&mut Ball, With<Gravity>>) {
	let delta_seconds = f32::min(0.2, time.delta_seconds());
	for mut ball in ball_query.iter_mut() {
		ball.velocity += Vec3::from([0.0, -980.0, 0.0]) * delta_seconds;
	}
}

fn platform_spawner_system(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	time: Res<Time>,
	mut timer: ResMut<PlatformTimer>,
	platform_query: Query<(Entity, &Transform), With<Platform>>,
	player_query: Query<&Transform, With<Ball>>,
) {
	let current_pos = player_query.single().unwrap().translation.x;

	for (platform, transform) in platform_query.iter() {
		if dbg!(transform.translation.x < current_pos - 500.0) {
			commands.entity(platform).despawn();
		}
	}

	if timer.0.tick(time.delta()).just_finished() {
		let y = thread_rng().gen_range(-225.0..-150.0);
		commands
			.spawn_bundle(SpriteBundle {
				material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
				transform: Transform::from_xyz(current_pos + 500.0, y, 0.0),
				sprite: Sprite::new(Vec2::new(500.0, 30.0)),
				..Default::default()
			})
			.insert(Collider)
			.insert(Platform);
	}
}

fn camera_tracking_system(
	mut queries: QuerySet<(
		Query<&Transform, With<Ball>>,
		Query<(&mut Transform, &Camera)>,
	)>,
) {
	let current_pos = queries.q0().single().unwrap().translation.x;
	for (mut transform, camera) in queries.q1_mut().iter_mut() {
		if camera.name.as_deref() == Some(CAMERA_2D) {
			transform.translation.x = current_pos;
		}
	}
}
