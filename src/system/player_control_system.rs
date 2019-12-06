use crate::resource::{Payload, Player, Position};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::network::{NetConnection, NetEvent, NetPacket};
use amethyst::prelude::*;

/**
 * Handling user input and move entity accordingly
*/
#[derive(SystemDesc)]
pub struct PlayerControlSystem;

impl<'a> System<'a> for PlayerControlSystem {
	type SystemData = (
		WriteStorage<'a, NetConnection<Payload>>,
		WriteStorage<'a, Position>,
		ReadStorage<'a, Player>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut net, mut position, players, input): Self::SystemData) {
		for (connection, position, _) in (&mut net, &mut position, &players).join() {
			let speed = 1.3;
			let position: &mut Position = position;
			let vertical = input.axis_value("player_vertical").unwrap_or_default();
			let horizontal = input.axis_value("player_horizontal").unwrap_or_default();
			let shoot = input.action_is_down("press_space").unwrap_or_default();

			if shoot {
				println!("Open fire!");
				let connection: &mut NetConnection<Payload> = connection;
				let packet = NetEvent::Packet(
					NetPacket::unreliable(
						Payload::new("boomber", "Spacebar Pressed!")
					)
				);
				connection.queue(packet);
			}

			// ! Vector is flipped for some reason when use `set_translation()`
			position.vel.x = horizontal * speed;
			position.vel.y = vertical * speed;
		}
	}
}
