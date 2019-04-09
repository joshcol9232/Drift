use raylib::Vector2;

use crate::misc;
use crate::car::COM_OFF;

const BACK_WHEEL_X_OFF: f32 = 2.0;
const BACK_WHEEL_Y_OFF: f32 = 7.0;
const FRONT_WHEEL_X_OFF: f32 = 2.0;
const FRONT_WHEEL_Y_OFF: f32 = 2.0;

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
	pub fn new(pos: Vector2, half_w: f32, half_h: f32, rotation: f32, time: f64) -> DriftTrailSet {
		DriftTrailSet {
			left_front: misc::rotate_vec(Vector2 { x: -half_w + FRONT_WHEEL_X_OFF, y: half_h - COM_OFF - FRONT_WHEEL_Y_OFF }, rotation) + pos,
			right_front: misc::rotate_vec(Vector2 { x: half_w - FRONT_WHEEL_X_OFF, y: half_h - COM_OFF - FRONT_WHEEL_Y_OFF }, rotation) + pos,
			left_back: misc::rotate_vec(Vector2 { x: -half_w + BACK_WHEEL_X_OFF, y: -half_h - COM_OFF + BACK_WHEEL_Y_OFF }, rotation) + pos,
			right_back: misc::rotate_vec(Vector2 { x: half_w - BACK_WHEEL_X_OFF, y: -half_h - COM_OFF + BACK_WHEEL_Y_OFF }, rotation) + pos,
			time_created: time
		}
	}
}
