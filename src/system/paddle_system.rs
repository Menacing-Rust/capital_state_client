use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::resource::{Paddle, Side};

#[derive(SystemDesc, Default)]
pub struct PaddleSystem;

impl<'a> System<'a> for PaddleSystem {
	type SystemData = (
		WriteStorage<'a, Transform>,
		ReadStorage<'a, Paddle>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
		for (paddle, transform) in (&paddles, &mut transforms).join() {
			let movement = match paddle.side {
				Side::Left => input.axis_value("left_paddle"),
				Side::Right => input.axis_value("right_paddle"),
			};

			if let Some(movement) = movement {
				let transform: &mut Transform = transform;
				transform.prepend_translation_y(movement * 1.3);
			}
		}
	}
}
