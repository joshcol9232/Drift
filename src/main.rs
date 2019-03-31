use raylib::{Color, Vector2, RaylibHandle};
 
mod car;
mod pillar;
mod misc;

static BG_COLOR: Color = Color { r: 230, g: 230, b: 220, a: 255 };
static RED_1: Color = Color { r: 190, g: 36, b: 25, a: 255 };
static RED_2: Color = Color { r: 232, g: 89, b: 79, a: 255 };
static CHARCOAL: Color = Color { r: 46, g: 46, b: 46, a: 255 };

struct Game {
	player: car::Car,
	pillars: Vec<pillar::Pillar>,
	score: u32
}

impl Default for Game {
	fn default() -> Game {
		Game {
			player: car::Car::new(Vector2 { x: 300.0, y: 300.0 }),
			pillars: vec![pillar::Pillar::default()],
			score: 0
		}
	}
}

impl Game {
	pub fn new(p: car::Car) -> Game {
		Game { player: p, pillars: vec![], ..Default::default() }
	}

	pub fn draw(&self, rl: &RaylibHandle) {
		for p in self.pillars.iter() {
			p.draw(rl);
		}

		self.player.draw(rl);
	}

	pub fn update(&mut self, rl: &RaylibHandle, dt: f32) {
		self.player.update(rl, dt);
		let low = self.get_closest_pillar_to_player();
	}

	fn get_closest_pillar_to_player(&self) -> (i32, f32) {
		let mut lowest = (-1, -1.0);
		for (i, p) in self.pillars.iter().enumerate() {
			let dist = p.distance_to(self.player.pos);
			if dist < lowest.1 || lowest.0 == -1 {
				lowest = (i as i32, dist);
			}
		}
		lowest
	}
}

fn main() {
	let rl = raylib::init()
			.size(1000, 800)
			.title("Drift")
			.build();

	rl.set_target_fps(144);

	let mut g = Game::default();

	while !rl.window_should_close() {
		g.update(&rl, rl.get_frame_time());

		rl.begin_drawing();
		rl.clear_background(BG_COLOR);
		g.draw(&rl);

		rl.draw_fps(10, 10);
		rl.end_drawing();
	}
}

