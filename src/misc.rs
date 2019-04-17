use raylib::Vector2;

pub fn get_components(mag: f32, angle: f32) -> Vector2 {
	Vector2 { x: mag * angle.sin(), y: mag * angle.cos() }
}

pub fn rotate_vec(v: Vector2, angle: f32) -> Vector2 {
	let a_c = angle.cos();
	let a_s = angle.sin();

	Vector2 { x: (v.x * a_c) - (v.y * a_s), y: (v.x * a_s) + (v.y * a_c) }
}
