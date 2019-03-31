use raylib::{Vector2, Rectangle};

use crate::misc;

pub struct DriftTrailSet {
	pub left_front: Vector2,
	pub right_front: Vector2,
	pub left_back: Vector2,
	pub right_back: Vector2,
	pub time_created: f64
}

impl Default for DriftTrailSet {
	fn default() -> DriftTrailSet {
		DriftTrailSet {
			left_front: Vector2 { x: -100.0, y: -100.0 },
			right_front: Vector2 { x: -100.0, y: -100.0 },
			left_back: Vector2 { x: -100.0, y: -100.0 },
			right_back: Vector2 { x: -100.0, y: -100.0 },
			time_created: 0.0
		}
	}
}

impl DriftTrailSet {
	pub fn new(pos: Vector2, half_w: f32, half_h: f32, rotation: f32) -> DriftTrailSet {
		let d = DriftTrailSet {
			left_front: misc::rotate_vec(Vector2 { body. }, rotation),
		}
	}
}
