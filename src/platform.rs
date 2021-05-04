use bevy::prelude::*;
use rand::prelude::*;

use crate::{player::Player, Collider, Scorable};

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.insert_resource(PlatformTimer::default())
			.add_startup_system(platform_setup_system.system())
			.add_system(platform_spawner_system.system());
	}
}

const PLAT_MIN: f32 = -225.0;
const PLAT_MAX: f32 = -150.0;
const PLAT_LEN: f32 = 500.0;
const PLAT_HGT: f32 = 30.0;

struct Platform;
struct PlatformTimer(Timer);

pub fn platform_setup_system(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
	Platform::spawn(
		&mut commands,
		&mut materials,
		Transform::from_xyz(PLAT_LEN / 2.0, PLAT_MIN, 0.0),
	)
}

fn platform_spawner_system(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	time: Res<Time>,
	mut timer: ResMut<PlatformTimer>,
	platform_query: Query<(Entity, &Transform), With<Platform>>,
	player_query: Query<&Transform, With<Player>>,
) {
	let current_pos = player_query.single().unwrap().translation.x;

	for (platform, transform) in platform_query.iter() {
		if !((current_pos - 500.0)..(current_pos + 500.0)).contains(&transform.translation.x) {
			commands.entity(platform).despawn();
		}
	}

	if timer.0.tick(time.delta()).just_finished() {
		let y = thread_rng().gen_range(PLAT_MIN..PLAT_MAX);
		Platform::spawn(
			&mut commands,
			&mut materials,
			Transform::from_xyz(current_pos + 500.0, y, 0.0),
		)
	}
}

impl Platform {
	fn spawn(
		commands: &mut Commands,
		materials: &mut ResMut<Assets<ColorMaterial>>,
		location: Transform,
	) {
		commands
			.spawn_bundle(SpriteBundle {
				material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
				transform: location,
				sprite: Sprite::new(Vec2::new(PLAT_LEN, PLAT_HGT)),
				..Default::default()
			})
			.insert(Collider)
			.insert(Platform)
			.insert(Scorable(Some(10)));
	}
}

impl Default for PlatformTimer {
	fn default() -> Self {
		Self(Timer::from_seconds(2.0, true))
	}
}
