use raylib::{Color, Vector2, RaylibHandle};
 
mod car;
mod misc;

struct Game {
	player: car::Car
}

impl Default for Game {
	fn default() -> Game {
		Game { player: car::Car::new(Vector2 { x: 300.0, y: 300.0 }) }
	}
}

impl Game {
	pub fn new(p: car::Car) -> Game {
		Game { player: p }
	}

	pub fn draw(&self, rl: &RaylibHandle) {
		self.player.draw(rl);
	}

	pub fn update(&mut self, rl: &RaylibHandle, dt: f32) {
		self.player.update(rl, dt);
	}
}

fn main() {
		let rl = raylib::init()
				.size(1000, 800)
				.title("Idk")
				.build();

		rl.set_target_fps(144);

		let bg_color = Color { r: 230, g: 230, b: 220, a: 255 };
		let mut g = Game::default();

		while !rl.window_should_close() {
				g.update(&rl, rl.get_frame_time());

				rl.begin_drawing();
				rl.clear_background(bg_color);
				g.draw(&rl);

				rl.draw_fps(10, 10);
				rl.end_drawing();
		}
}

