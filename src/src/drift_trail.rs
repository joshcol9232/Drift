use raylib::math::Vector2;

pub struct DriftTrailSet {
	pub left_front: Vector2,
	pub right_front: Vector2,
	pub left_back: Vector2,
	pub right_back: Vector2,
	pub time_created: f64
}

impl DriftTrailSet {
	pub fn new(time: f64, wheel_positions: &[Vector2; 4]) -> DriftTrailSet {
		DriftTrailSet {
			left_front: wheel_positions[0],
			right_front: wheel_positions[1],
			left_back: wheel_positions[2],
			right_back: wheel_positions[3],
			time_created: time
		}
	}
}
