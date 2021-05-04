use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.insert_resource(Scoreboard { score: 0 })
			.add_startup_system(score_setup_system.system())
			.add_system(scoreboard_system.system());
	}
}

pub struct Scoreboard {
	pub score: usize,
}

pub struct Scorable(pub Option<usize>);

fn score_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
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
