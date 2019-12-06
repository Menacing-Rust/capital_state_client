use amethyst::ecs::{Component, DenseVecStorage};

pub struct Player {
	pub width: f32,
	pub height: f32
}

impl Player {
	pub fn new(width: f32, height: f32) -> Player {
		Player { width, height }
	}
}

impl Component for Player {
	type Storage = DenseVecStorage<Self>;
}