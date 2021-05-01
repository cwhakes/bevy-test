mod platform;

use bevy::{
	prelude::*,
	render::camera::Camera,
	render::pass::ClearColor,
	render::render_graph::base::camera::CAMERA_2D,
	sprite::collide_aabb::{collide, Collision},
};

use platform::PlatformPlugin;

/// An implementation of the classic game "Breakout"
fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		.add_plugin(PlatformPlugin)
		.insert_resource(Scoreboard { score: 0 })
		.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
		.add_startup_system(setup.system())
		.add_system(ball_collision_system.system())
		.add_system(ball_movement_system.system())
		.add_system(ball_gravity_system.system())
		.add_system(scoreboard_system.system())
		.add_system(camera_tracking_system.system())
		.add_system(ball_control_system.system())
		.run();
}

struct Ball {
	velocity: Vec3,
}

struct Scoreboard {
	score: usize,
}

struct Gravity;

struct Collider;

struct Scorable(Option<usize>);

fn setup(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	asset_server: Res<AssetServer>,
) {
	// Add the game's entities to our world

	// cameras
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands.spawn_bundle(UiCameraBundle::default());
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
	mut scoreboard: ResMut<Scoreboard>,
	mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
	mut collider_query: Query<(&Transform, &Sprite, Option<&mut Scorable>), With<Collider>>,
) {
	if let Ok((mut ball, ball_transform, sprite)) = ball_query.single_mut() {
		let ball_size = sprite.size;
		let velocity = &mut ball.velocity;

		// check collision with walls
		for (transform, sprite, scorable) in collider_query.iter_mut() {
			let collision = collide(
				ball_transform.translation,
				ball_size,
				transform.translation,
				sprite.size,
			);
			if let Some(collision) = collision {

				if let Some(mut scorable) = scorable {
					if let Some(score) = scorable.0.take() {
						scoreboard.score += score;
					}
				}

				if let Collision::Top = collision {
					velocity.y = velocity.y.max(0.0);
				}
			}
		}
	}
}

fn ball_control_system(
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&mut Ball, &Transform, &Sprite)>,
	collider_query: Query<(&Transform, &Sprite), With<Collider>>,
) {
	if let Ok((mut ball, ball_transform, sprite)) = query.single_mut() {
		if keyboard_input.just_pressed(KeyCode::Space) {
			let ball_size = sprite.size;
			let velocity = &mut ball.velocity;

			for (transform, sprite) in collider_query.iter() {
				let collision = collide(
					ball_transform.translation,
					ball_size,
					transform.translation,
					sprite.size,
				);

				if collision.is_some() {
					velocity.y += 500.0;
					break;
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
