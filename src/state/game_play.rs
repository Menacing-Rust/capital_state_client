use super::Pause;
use amethyst::ecs::{Dispatcher, DispatcherBuilder, Entity, World};
use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    core::ArcThreadPool,
    input::InputEvent,
    network::*,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    winit::VirtualKeyCode,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

use crate::resource::{Payload, Player, Position};
use crate::system::{MovementSystem, PlayerControlSystem};

const WORLD_WIDTH: f32 = 100.0;
const WORLD_HEIGHT: f32 = 100.0;

pub struct GamePlay {
    dispatcher: Dispatcher<'static, 'static>,
    camera: Option<Entity>,
    player: Option<Entity>,
}

impl GamePlay {
    pub fn new(world: &mut World) -> GamePlay {
        let pool = (*world.read_resource::<ArcThreadPool>()).clone();
        GamePlay {
            dispatcher: DispatcherBuilder::new()
                .with_pool(pool)
                .with(MovementSystem, "movement", &[])
                .with(PlayerControlSystem, "player_control", &[])
                .build(),
            camera: None,
            player: None,
        }
    }
}

impl SimpleState for GamePlay {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.dispatcher.setup(world);

        let host_ip: SocketAddr = SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 7070);
        world.register::<Player>();

        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "textures/players.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };
        let sprite_sheet = {
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                "textures/players.ron",
                SpriteSheetFormat(texture_handle),
                (),
                &sprite_sheet_storage,
            )
        };
        let camera = Camera::standard_2d(WORLD_WIDTH, WORLD_HEIGHT);
        let mut camera_transform = Transform::default();
        camera_transform.set_translation_xyz(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0, 1.0);

        let player = Player::new(30.0, 30.0);
        let player_position = Position::from(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0, 0.0);
        self.camera = Some(
            world
                .create_entity()
                .with(camera)
                .with(camera_transform)
                .build(),
        );
        self.player = Some(
            world
                .create_entity()
                .with(player)
                .with(player_position.transform())
                .with(player_position)
                .with(NetConnection::<Payload>::new(host_ip))
                .with(SpriteRender {
                    sprite_sheet: sprite_sheet.clone(),
                    sprite_number: 0,
                })
                .build(),
        );
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let entities = [self.camera.unwrap(), self.player.unwrap()];
        data.world.delete_entities(&entities).unwrap();
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Input(input_event) => match input_event {
                InputEvent::KeyPressed { key_code, .. } => {
                    if key_code == VirtualKeyCode::Escape {
                        Trans::Push(Box::new(Pause::default()))
                    } else {
                        Trans::None
                    }
                }
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        self.dispatcher.dispatch(&data.world);
        Trans::None
    }
}
