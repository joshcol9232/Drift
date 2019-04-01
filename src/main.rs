use raylib::{Color, Vector2, RaylibHandle, Rectangle};
 
mod car;
mod drift_trail;
mod pillar;
mod misc;

static BG_COLOR: Color = Color { r: 230, g: 230, b: 220, a: 255 };
static RED_1: Color = Color { r: 190, g: 36, b: 25, a: 255 };
static RED_2: Color = Color { r: 232, g: 89, b: 79, a: 255 };
static CHARCOAL: Color = Color { r: 38, g: 38, b: 38, a: 255 };

const POINT_DIST_THRESHOLD: f32 = 100.0;
const DRIFT_TRAIL_WIDTH: f32 = 3.5;
const TRAIL_DURATION: f64 = 3.5; // In seconds
const MAX_POINTS_PER_FRAME: u32 = 500;

struct Game {
	player: car::Car,
	pillars: Vec<pillar::Pillar>,
	trail_nodes: Vec<drift_trail::DriftTrailSet>,
	score: u32
}

impl Default for Game {
	fn default() -> Game {
		Game {
			player: car::Car::new(Vector2 { x: 300.0, y: 300.0 }),
			pillars: vec![pillar::Pillar::default(), pillar::Pillar::new(Vector2 { x: 700.0, y: 400.0 }, 7.0)],
			trail_nodes: vec![],
			score: 0
		}
	}
}

impl Game {
	pub fn new(p: car::Car) -> Game {
		Game {
			player: p,
			pillars: vec![],
			..Default::default()
		}
	}

	pub fn draw(&self, rl: &RaylibHandle) {
		self.draw_trails(rl, rl.get_time());

		for p in self.pillars.iter() {
			//rl.draw_circle_v(p.pos, POINT_DIST_THRESHOLD, Color { r: 30, g: 160, b: 10, a: 100 });
			p.draw(rl);
		}

		self.player.draw(rl);

		rl.draw_text(format!("Score: {}", self.score).as_str(), 400, 10, 20, RED_2);
	}

	pub fn update(&mut self, rl: &RaylibHandle, dt: f32) {
		let curr_time = rl.get_time();

		self.remove_dead_trail_nodes(curr_time);
		self.player.update(rl, dt);

		if self.player.drifting {
			self.place_trails(curr_time);
			self.update_points(dt);
		}
	}

	fn update_points(&mut self, dt: f32) {
		let closest = self.get_closest_pillar_to_player();
		if closest.1 <= POINT_DIST_THRESHOLD {
			self.score += self.get_points_from_dist(dt, closest.1);
		}
	}

	fn get_points_from_dist(&mut self, dt: f32, dist: f32) -> u32 {
		(dt * (dist/POINT_DIST_THRESHOLD) * MAX_POINTS_PER_FRAME as f32) as u32
	}

	fn get_closest_pillar_to_player(&self) -> (i32, f32) {
		let mut closest = (0, -1.0);
		for (i, p) in self.pillars.iter().enumerate() {
			let dist = p.distance_to(self.player.pos);
			if dist < closest.1 || closest.1 < 0.0 {
				closest = (i as i32, dist);
			}
		}
		closest
	}

	fn remove_dead_trail_nodes(&mut self, time: f64) {
		self.trail_nodes.retain(|i|time - i.time_created <= TRAIL_DURATION);
	}

	fn place_trails(&mut self, time: f64) {
		self.trail_nodes.push(drift_trail::DriftTrailSet::new(self.player.pos, car::TRAIL_DRAW_W, car::TRAIL_DRAW_H, -self.player.angle, time));
	}

	fn draw_trails(&self, rl: &RaylibHandle, time: f64) {
		let mut last: &drift_trail::DriftTrailSet = &drift_trail::DriftTrailSet::default();
		for (i, t) in self.trail_nodes.iter().enumerate() {
			if i > 0 && last.left_front.distance_to(t.left_front) < 10.0 {
				let mut col = CHARCOAL;
				col.a = ((3.0 * ((t.time_created - time)/TRAIL_DURATION) + 4.0).log2() * 255.0).min(255.0) as u8;
				rl.draw_line_ex(last.left_front, t.left_front, DRIFT_TRAIL_WIDTH, col);  // Left front
				rl.draw_line_ex(last.right_front, t.right_front, DRIFT_TRAIL_WIDTH, col);  // Right front
				rl.draw_line_ex(last.left_back, t.left_back, DRIFT_TRAIL_WIDTH, col);  // Left back
				rl.draw_line_ex(last.right_back, t.right_back, DRIFT_TRAIL_WIDTH, col);  // Right back
			}

			last = t;
		}
	}
}

fn main() {
	let rl = raylib::init()
			.size(1000, 800)
			.title("Drift")
			.msaa_4x()
			.build();

	rl.set_target_fps(60);

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

