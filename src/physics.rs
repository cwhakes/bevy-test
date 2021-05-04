use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_system(movement_system.system())
			.add_system(gravity_system.system());
	}
}

pub struct Physics {
	velocity: Vec3,
}

pub struct Gravity(f32);

fn movement_system(time: Res<Time>, mut query: Query<(&Physics, &mut Transform)>) {
	// clamp the timestep to stop the object from escaping when the game starts
	let delta_seconds = f32::min(0.2, time.delta_seconds());

	if let Ok((object, mut transform)) = query.single_mut() {
		transform.translation += object.velocity * delta_seconds;
	}
}

fn gravity_system(time: Res<Time>, mut query: Query<(&mut Physics, &Gravity)>) {
	let delta_seconds = f32::min(0.2, time.delta_seconds());
	for (mut object, gravity) in query.iter_mut() {
		object.velocity += Vec3::from([0.0, -gravity.0, 0.0]) * delta_seconds;
	}
}

impl Default for Gravity {
	fn default() -> Self {
		Self(980.0)
	}
}
