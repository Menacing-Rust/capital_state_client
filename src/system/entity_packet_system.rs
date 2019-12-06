use crate::resource::Position;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};
use amethyst::network::{NetConnection, NetEvent, NetPacket};

/**
 * Sending entity info to the server
*/
pub struct EntityPacketSystem;

impl<'a> System<'a> for EntityPacketSystem {
	type SystemData = (
		WriteStorage<'a, NetConnection<String>>,
		ReadStorage<'a, Position>,
	);

	fn run(&mut self, (mut net, positions): Self::SystemData) {
		for (net, position) in (&mut net, &positions).join() {
			let net: &mut NetConnection<String> = net;
			let packet = NetEvent::Packet(
				NetPacket::unreliable(
					format!("Player: {:?}", position.pos)
				)
			);
			net.queue(packet);
		}
	}
}
