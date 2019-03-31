use raylib::Vector2;

pub fn get_components(mag: f32, angle: f32) -> Vector2 {
	Vector2 { x: mag * angle.sin(), y: mag * angle.cos() }
}

pub fn rotate_vec(vIn: Vector2, origin: Vector2, angle: f32) -> Vector2 {
	let mut temp = Vector2::zero();
	let a_c = angle.cos();
	let a_s = angle.sin();

	let v = origin - vIn;

	temp.x = (v.x * a_c) - (v.y * a_s);
	temp.y = (v.x * a_s) + (v.y * a_c);

	v + temp
}
