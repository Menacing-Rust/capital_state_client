use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, System, SystemData, World, WriteStorage};

use crate::resource::{Position};

/**
 * Handling entity's transform of Position component
*/
#[derive(SystemDesc, Default)]
pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
	type SystemData = (WriteStorage<'a, Transform>, WriteStorage<'a, Position>);

	fn run(&mut self, (mut transforms, mut position): Self::SystemData) {
		for (transform, position) in (&mut transforms, &mut position).join() {
			let position: &mut Position = position;
			let transform: &mut Transform = transform;

			position.pos += position.vel;
			position.vel += position.acc;

			transform.set_translation(position.pos);
		}
	}
}
