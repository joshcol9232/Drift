use raylib::Vector2;

use crate::misc;
use crate::car::COM_OFF;

pub struct DriftTrailSet {
	pub left_front: Vector2,
	pub right_front: Vector2,
	pub left_back: Vector2,
	pub right_back: Vector2,
	pub time_created: f64
}

impl Default for DriftTrailSet {
	fn default() -> DriftTrailSet {   // For initializing stuff
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
	pub fn new(pos: Vector2, time: f64, wheel_positions: &[Vector2; 4]) -> DriftTrailSet {
		DriftTrailSet {
			left_front: wheel_positions[0],
			right_front: wheel_positions[1],
			left_back: wheel_positions[2],
			right_back: wheel_positions[3],
			time_created: time
		}
	}
}
