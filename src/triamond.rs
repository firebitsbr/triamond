use amethyst::{
	core::transform::Transform,
	prelude::*,
	renderer::Camera,
};

use crate::math::tau;

pub struct Triamond;

impl SimpleState for Triamond {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;
		initialise_camera(world);
	}
}

fn initialise_camera(world: &mut World) {
	let mut transform = Transform::default();
	transform
		.set_translation_xyz(-10.0, 7.5, 7.5)
		.set_rotation_euler(0.0, tau() / 12.0, tau() / 12.0);

	world
		.create_entity()
		.with(Camera::standard_3d(1280.0, 720.0))
		.with(transform)
		.build();
}
