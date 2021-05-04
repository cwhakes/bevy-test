mod physics;
mod platform;
mod player;
mod score;

use bevy::{prelude::*, render::pass::ClearColor};

use physics::PhysicsPlugin;
use platform::PlatformPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;

/// An implementation of the classic game "Breakout"
fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		.add_plugin(PhysicsPlugin)
		.add_plugin(PlatformPlugin)
		.add_plugin(PlayerPlugin)
		.add_plugin(ScorePlugin)
		.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
		.add_startup_system(setup.system())
		.run();
}

struct Collider;

fn setup(mut commands: Commands) {
	// cameras
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands.spawn_bundle(UiCameraBundle::default());
}
