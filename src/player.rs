use bevy::{
	prelude::*,
	render::camera::Camera,
	render::render_graph::base::camera::CAMERA_2D,
	sprite::collide_aabb::{collide, Collision},
};

use crate::{Collider, Gravity, Scorable, Scoreboard};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
			.add_startup_system(player_setup_system.system())
			.add_system(player_collision_system.system())
			.add_system(player_movement_system.system())
			.add_system(player_gravity_system.system())
			.add_system(camera_tracking_system.system())
			.add_system(player_control_system.system())
			.add_system(death_system.system());
	}
}

pub struct Player {
	velocity: Vec3,
}

fn player_setup_system(	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,) {
	commands
	.spawn_bundle(SpriteBundle {
		material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
		transform: Transform::from_xyz(0.0, -50.0, 1.0),
		sprite: Sprite::new(Vec2::new(30.0, 30.0)),
		..Default::default()
	})
	.insert(Player {
		velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
	})
	.insert(Gravity);
}

fn player_movement_system(time: Res<Time>, mut player_query: Query<(&Player, &mut Transform)>) {
	// clamp the timestep to stop the player from escaping when the game starts
	let delta_seconds = f32::min(0.2, time.delta_seconds());

	if let Ok((player, mut transform)) = player_query.single_mut() {
		transform.translation += player.velocity * delta_seconds;
	}
}

fn player_collision_system(
	mut scoreboard: ResMut<Scoreboard>,
	mut player_query: Query<(&mut Player, &Transform, &Sprite)>,
	mut collider_query: Query<(&Transform, &Sprite, Option<&mut Scorable>), With<Collider>>,
) {
	if let Ok((mut player, player_transform, sprite)) = player_query.single_mut() {
		let player_size = sprite.size;
		let velocity = &mut player.velocity;

		// check collision with walls
		for (transform, sprite, scorable) in collider_query.iter_mut() {
			let collision = collide(
				player_transform.translation,
				player_size,
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

fn player_control_system(
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&mut Player, &Transform, &Sprite)>,
	collider_query: Query<(&Transform, &Sprite), With<Collider>>,
) {
	if let Ok((mut player, player_transform, sprite)) = query.single_mut() {
		if keyboard_input.just_pressed(KeyCode::Space) {
			let player_size = sprite.size;
			let velocity = &mut player.velocity;

			for (transform, sprite) in collider_query.iter() {
				let collision = collide(
					player_transform.translation,
					player_size,
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

fn player_gravity_system(time: Res<Time>, mut player_query: Query<&mut Player, With<Gravity>>) {
	let delta_seconds = f32::min(0.2, time.delta_seconds());
	for mut player in player_query.iter_mut() {
		player.velocity += Vec3::from([0.0, -980.0, 0.0]) * delta_seconds;
	}
}

fn camera_tracking_system(
	mut queries: QuerySet<(
		Query<&Transform, With<Player>>,
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

fn death_system(
	commands: Commands,
	materials: ResMut<Assets<ColorMaterial>>,
	mut scoreboard: ResMut<Scoreboard>,
	mut player_query: Query<(&mut Player, &mut Transform)>,
) {
	if let Ok((mut player, mut transform)) = player_query.single_mut() {
		if transform.translation.y < -500.0 {
			player.velocity = 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize();
			*transform = Transform::from_xyz(0.0, -50.0, 1.0);
			scoreboard.score = 0;

			crate::platform::platform_setup_system(commands, materials);
		}
	}
}

