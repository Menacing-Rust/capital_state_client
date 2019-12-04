use amethyst::{
	prelude::*,
	ecs::Entity,
	input::{is_close_requested, is_key_down},
	ui::{UiCreator, UiEventType},
	winit::{VirtualKeyCode},
};
use super::Menu;
use super::delete_hierarchy;

#[derive(Default)]
pub struct GameOver {
	handle: Option<Entity>,
}

impl SimpleState for GameOver {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;
		self.handle = Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/game_over.ron", ())));
	}
	
	fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
		match &event {
			StateEvent::Window(event) => {
				if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
					Trans::Quit
				}
				else {
					Trans::None
				}
			},
			StateEvent::Ui(event) => {
				match event.event_type {
					UiEventType::Click => Trans::Switch(Box::new(Menu::default())),
					_ => Trans::None
				}
			},
			_ => Trans::None
		}
	}

	fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		if let Some(handle) = self.handle {
			delete_hierarchy(handle, data.world).unwrap();
		}
	}
}