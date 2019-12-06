// #![windows_subsystem = "windows"]

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{InputBundle, StringBindings},
    network::*,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

mod resource;
mod state;
mod system;
use state::*;
use resource::Payload;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let host_ip: SocketAddr = SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 7070);

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
        .with_bundle(render_bundle)?
        .with_bundle(NetworkBundle::<Payload>::new(host_ip))?
        // .with(system::EntityPacketSystem, "entity_packet_system", &[])
        .with(system::ReceiverSystem, "network_receiver", &[]);
    let mut game = Application::build(assets, Menu::default())?
        .with_frame_limit(FrameRateLimitStrategy::Sleep, 60)
        .build(game_data)?;
    game.run();
    Ok(())
}
