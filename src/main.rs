mod physics;
mod platform;
mod player;

use bevy::{
	prelude::*,
	render::pass::ClearColor,
};

use physics::PhysicsPlugin;
use platform::PlatformPlugin;
use player::PlayerPlugin;

/// An implementation of the classic game "Breakout"
fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		.add_plugin(PhysicsPlugin)
		.add_plugin(PlatformPlugin)
		.add_plugin(PlayerPlugin)
		.insert_resource(Scoreboard { score: 0 })
		.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
		.add_startup_system(setup.system())
		.add_system(scoreboard_system.system())
		.run();
}



struct Scoreboard {
	score: usize,
}

struct Collider;

struct Scorable(Option<usize>);

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	// Add the game's entities to our world

	// cameras
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands.spawn_bundle(UiCameraBundle::default());

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

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
	let mut text = query.single_mut().unwrap();
	text.sections[0].value = format!("Score: {}", scoreboard.score);
}


