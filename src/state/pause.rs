use amethyst::ecs::WorldExt;
use amethyst::{
	ecs::Entity,
	input::InputEvent,
	prelude::*,
	shrev::EventChannel,
	ui::{UiCreator, UiEvent, UiEventType, UiFinder},
	winit::VirtualKeyCode,
};

use super::delete_hierarchy;
use super::menu::Menu;

#[derive(Default)]
pub struct Pause {
	handle: Option<Entity>,
	continue_button: Option<Entity>,
	main_menu_button: Option<Entity>,
}

impl SimpleState for Pause {
	fn on_start(&mut self, data: StateData<GameData>) {
		let world = data.world;
		let handle = world.exec(|mut creator: UiCreator<'_>| creator.create("ui/pause.ron", ()));
		self.handle = Some(handle);
	}

	fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
		if self.continue_button.is_none() || self.main_menu_button.is_none() {
			data.world.exec(|finder: UiFinder| {
				self.continue_button = finder.find("continue_button");
				self.main_menu_button = finder.find("main_menu_button");
			});
		}
		Trans::None
	}

	fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
		match event {
			StateEvent::Ui(UiEvent {
				event_type: UiEventType::Click,
				target,
			}) => {
				if Some(target) == self.continue_button {
					Trans::Pop
				} else if Some(target) == self.main_menu_button {
					/*
						TODO: Fix this shit
						Expected behavior: Popping Pause state and then Switching GamePlay state to Menu state
					*/
					let mut resource_writer = data.world.write_resource::<EventChannel<TransEvent<GameData, StateEvent>>>();
					resource_writer.single_write(Box::new(|| Trans::Pop));
					resource_writer.single_write(Box::new(|| Trans::Switch(Box::new(Menu::default()))));
					Trans::None
				} else {
					Trans::None
				}
			}
			StateEvent::Input(event) => match event {
				InputEvent::KeyPressed { key_code, .. } => match key_code {
					VirtualKeyCode::Escape => Trans::Pop,
					_ => Trans::None,
				},
				_ => Trans::None,
			},
			_ => Trans::None,
		}
	}

	fn on_stop(&mut self, data: StateData<GameData>) {
		if let Some(handle) = self.handle {
			delete_hierarchy(handle, data.world).unwrap();
		}
	}
}
