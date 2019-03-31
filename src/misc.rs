use raylib::Vector2;

pub fn get_components(mag: f32, angle: f32) -> Vector2 {
	Vector2 { x: mag * angle.sin(), y: mag * angle.cos() }
}

pub fn rotate_vec(v: Vector2, angle: f32) -> Vector2 {
	let temp = Vector2::zero();
	let ac = angle.cos();
	let as = angle.sin();

	temp.x = (v.x * ac) - (v.y * as);
	temp.y = (v.x * as) + (v.y * ac);

	v + temp
}
