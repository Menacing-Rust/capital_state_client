use super::{Pause};
use amethyst::{
	assets::{AssetStorage, Loader},
	core::transform::Transform,
	ecs::prelude::*,
	input::InputEvent,
	prelude::*,
	renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
	winit::VirtualKeyCode,
};

use crate::resource::Paddle;
use crate::system::PaddleSystem;

const WORLD_WIDTH: f32 = 100.0;
const WORLD_HEIGHT: f32 = 100.0;

const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 30.0;

#[derive(Default)]
pub struct GamePlay {
	dispatcher: Option<Dispatcher<'static, 'static>>,
	camera: Option<Entity>,
	left_paddle: Option<Entity>,
	right_paddle: Option<Entity>,
}

impl SimpleState for GamePlay {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;

		world.register::<Paddle>();

		let texture_handle = {
			let loader = world.read_resource::<Loader>();
			let texture_storage = world.read_resource::<AssetStorage<Texture>>();
			loader.load(
				"textures/paddles.png",
				ImageFormat::default(),
				(),
				&texture_storage,
			)
		};
		let sprite_sheet = {
			let loader = world.read_resource::<Loader>();
			let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
			loader.load(
				"textures/paddles.ron",
				SpriteSheetFormat(texture_handle),
				(),
				&sprite_sheet_storage,
			)
		};
		let camera = Camera::standard_2d(WORLD_WIDTH, WORLD_HEIGHT);
		let mut camera_transform = Transform::default();
		camera_transform.set_translation_xyz(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0, 1.0);

		let (left_paddle, right_paddle) = Paddle::pair(PADDLE_WIDTH, PADDLE_HEIGHT);
		let mut left_transform = Transform::default();
		let mut right_transform = Transform::default();
		left_transform.set_translation_xyz(left_paddle.width / 2.0, WORLD_HEIGHT / 2.0, 0.0);
		right_transform.set_translation_xyz(
			WORLD_WIDTH - right_paddle.width / 2.0,
			WORLD_HEIGHT / 2.0,
			0.0,
		);
		self.camera = Some(
			world
				.create_entity()
				.with(camera)
				.with(camera_transform)
				.build(),
		);
		self.left_paddle = Some(
			world
				.create_entity()
				.with(left_paddle)
				.with(left_transform)
				.with(SpriteRender {
					sprite_sheet: sprite_sheet.clone(),
					sprite_number: 0,
				})
				.build(),
		);
		self.right_paddle = Some(
			world
				.create_entity()
				.with(right_paddle)
				.with(right_transform)
				.with(SpriteRender {
					sprite_sheet: sprite_sheet.clone(),
					sprite_number: 1,
				})
				.build(),
		);

		let mut dispatcher_builder = DispatcherBuilder::new();
		dispatcher_builder.add(PaddleSystem, "paddle_system", &[]);
		let mut dispatcher = dispatcher_builder.build();
		dispatcher.setup(world);
		self.dispatcher = Some(dispatcher);
	}

	fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
		if let Some(dispatcher) = self.dispatcher.as_mut() {
			dispatcher.dispatch(&data.world);
		}
		Trans::None
	}

	fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
		match &event {
			StateEvent::Input(input_event) => match input_event {
				InputEvent::KeyPressed { key_code, .. } => {
					if key_code == &VirtualKeyCode::Escape {
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

	fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let entities = [self.camera.unwrap(), self.left_paddle.unwrap(), self.right_paddle.unwrap()];
		data.world.delete_entities(&entities).unwrap();
	}
}
