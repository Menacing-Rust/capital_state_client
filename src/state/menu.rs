use super::delete_hierarchy;
use super::GamePlay;
use amethyst::{
	ecs::Entity,
	input::{is_close_requested, is_key_down},
	prelude::*,
	ui::{UiCreator, UiEvent, UiEventType, UiFinder},
	winit::VirtualKeyCode,
};

#[derive(Default)]
pub struct Menu {
	handle: Option<Entity>,
	play_button: Option<Entity>,
}

impl SimpleState for Menu {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;
		self.handle = Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
	}

	fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
		let StateData { world, .. } = data;

		if self.play_button.is_none() {
			world.exec(|finder: UiFinder<'_>| {
				self.play_button = finder.find("play_button");
			});
		}

		Trans::None
	}

	fn handle_event(
		&mut self,
		data: StateData<'_, GameData<'_, '_>>,
		event: StateEvent,
	) -> SimpleTrans {
		match &event {
			StateEvent::Window(event) => {
				if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
					Trans::Quit
				} else {
					Trans::None
				}
			}
			StateEvent::Ui(UiEvent { event_type, target }) => match event_type {
				UiEventType::Click => {
					if Some(target) == self.play_button.as_ref() {
						Trans::Switch(Box::new(GamePlay::new(data.world)))
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
		if let Some(handle) = self.handle {
			delete_hierarchy(handle, data.world).unwrap();
		}
	}
}
