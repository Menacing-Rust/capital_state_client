use amethyst::ecs::{Component, VecStorage};

pub enum Side {
	Left,
	Right
}

pub struct Paddle {
	pub side: Side,
	pub width: f32,
	pub height: f32
}

impl Paddle {
	pub fn new(side: Side, width: f32, height: f32) -> Paddle {
		Paddle { side, width, height }
	}

	pub fn pair(width: f32, height: f32) -> (Paddle, Paddle) {
		(
			Paddle { side: Side::Left, width, height },
			Paddle { side: Side::Right, width, height }
		)
	}
}

impl Component for Paddle {
	type Storage = VecStorage<Self>;
}