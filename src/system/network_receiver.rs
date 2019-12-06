use crate::resource::Payload;
use amethyst::{
    derive::SystemDesc,
    ecs::{Component, DenseVecStorage, Entities, Join, ReaderId, System, SystemData, WriteStorage},
    network::{NetConnection, NetEvent},
    prelude::*,
};

pub struct ConnectionReader(ReaderId<NetEvent<Payload>>);

impl Component for ConnectionReader {
    type Storage = DenseVecStorage<Self>;
}

#[derive(SystemDesc)]
pub struct ReceiverSystem;

impl<'a> System<'a> for ReceiverSystem {
    type SystemData = (
        WriteStorage<'a, NetConnection<Payload>>,
        WriteStorage<'a, ConnectionReader>,
        Entities<'a>,
    );
    fn run(&mut self, (mut net, mut readers, entities): Self::SystemData) {
        for (connection, entity) in (&mut net, &entities).join() {
            let connection: &mut NetConnection<Payload> = connection;
            let reader = readers
                .entry(entity)
                .expect("Cannot get reader")
                .or_insert_with(|| ConnectionReader(connection.register_reader()));
            let mut disconnected = false;

            for event in connection.received_events(&mut reader.0) {
                match event {
                    NetEvent::Connected(address) => println!("{} > Connected", address),
                    NetEvent::Disconnected(address) => {
                        println!("{} > Disconnected", address);
                        disconnected = true;
                    }
                    NetEvent::Packet(packet) => println!("{:#?}", packet.content()),
                    _ => (),
                }
            }

            if disconnected {
                entities.delete(entity).expect("Cannot find entity");
            }
        }
    }
}
