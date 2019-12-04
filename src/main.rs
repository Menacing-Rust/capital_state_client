// #![windows_subsystem = "windows"]

use amethyst::{
	core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
	input::{InputBundle, StringBindings},
	prelude::*,
	renderer::{
		plugins::{RenderFlat2D, RenderToWindow},
		types::DefaultBackend,
		RenderingBundle,
	},
	ui::{RenderUi, UiBundle},
	utils::application_root_dir,
};
use amethyst::ecs::WorldExt;
use std::time::Duration;

mod resource;
mod state;
mod system;
use state::*;

fn main() -> amethyst::Result<()> {
	amethyst::start_logger(Default::default());

	let root = application_root_dir()?;
	let display = root.join("config").join("display.ron");
	let assets = root.join("assets");
	let input = root.join("config").join("input.ron");

	let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(input)?;
	let render_bundle = RenderingBundle::<DefaultBackend>::new()
		.with_plugin(RenderFlat2D::default())
		.with_plugin(RenderToWindow::from_config_path(display).with_clear([0.0, 0.0, 0.0, 1.0]))
		.with_plugin(RenderUi::default());

	let game_data = GameDataBuilder::default()
		.with_bundle(TransformBundle::new())?
		.with_bundle(UiBundle::<StringBindings>::new())?
		.with_bundle(input_bundle)?
		.with_bundle(render_bundle)?;
	let mut game = Application::build(assets, Menu::default())?
		.with_frame_limit(
			FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
			60,
		)
		.build(game_data)?;
	game.run();
	Ok(())
}
