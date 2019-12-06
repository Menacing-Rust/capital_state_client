use amethyst::core::math::Vector3;
use amethyst::ecs::prelude::*;
use amethyst::core::transform::Transform;

#[derive(Debug)]
pub struct Position {
	pub pos: Vector3<f32>,
	pub vel: Vector3<f32>,
	pub acc: Vector3<f32>,
}

impl Default for Position {
	fn default() -> Position {
		Position {
			pos: Vector3::<f32>::new(0.0, 0.0, 0.0),
			vel: Vector3::<f32>::new(0.0, 0.0, 0.0),
			acc: Vector3::<f32>::new(0.0, 0.0, 0.0)
		}
	}
}

impl Position {
	pub fn transform(&self) -> Transform {
		let transform = Transform::from(self.pos);
		transform
	}

	pub fn from(x: f32, y: f32, z: f32) -> Position {
		Position {
			pos: Vector3::<f32>::new(x, y, z),
			vel: Vector3::<f32>::new(0.0, 0.0, 0.0),
			acc: Vector3::<f32>::new(0.0, 0.0, 0.0),
		}
	}
}

impl Component for Position {
	type Storage = DenseVecStorage<Self>;
}