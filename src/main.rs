extern crate rand;

use raylib::{Color, Vector2, RaylibHandle, Texture2D, consts};

mod car;
mod drift_trail;
mod dust_system;
mod pillar;
mod misc;

static BG_COLOR: Color = Color { r: 230, g: 230, b: 220, a: 255 };
static RED_1: Color = Color { r: 190, g: 36, b: 25, a: 255 };
static RED_2: Color = Color { r: 232, g: 89, b: 79, a: 255 };
static CHARCOAL: Color = Color { r: 38, g: 38, b: 38, a: 255 };

const POINT_DIST_THRESHOLD: f32 = 100.0;
const MAX_POINTS_PER_FRAME: u32 = 5;

struct Game {
	player: car::Car,
	pillars: Vec<pillar::Pillar>,
   car_texture: Texture2D,
	score: u32
}

impl<'a> Game {
	pub fn new(p: car::Car, c_texture: Texture2D) -> Game {
		Game {
			player: p,
			pillars: vec![],
			car_texture: c_texture,
			score: 0
		}
	}

	pub fn draw(&self, rl: &RaylibHandle) {
		for p in self.pillars.iter() {
			p.draw(rl);
		}

		self.player.draw(&self.car_texture, rl);

		rl.draw_text(format!("Score: {}", self.score).as_str(), 400, 10, 20, RED_2);
		rl.draw_text(format!("Trail nodes: {}", self.player.get_trail_node_count()).as_str(), 10, 32, 20, CHARCOAL);
		rl.draw_text(format!("Player speed: {:.1}", self.player.vel_mag).as_str(), 10, 54, 20, CHARCOAL);
		rl.draw_text(format!("Player perp: {:.3}", self.player.perp).as_str(), 10, 76, 20, CHARCOAL);
		rl.draw_text(format!("Particle count: {}", self.player.get_particle_count()).as_str(), 10, 120, 20, CHARCOAL);
	}

	pub fn update(&mut self, rl: &RaylibHandle, dt: f32) {
		self.player.update(rl, dt);

		if self.player.drifting {
			self.update_points(dt);
		}

		if rl.is_key_pressed(consts::KEY_R as i32) { self.reload() }
	}

	fn reload(&mut self) {
		self.player.reset();
		self.score = 0;
	}

	pub fn add_pillar(&mut self, p: Vector2, r: f32) {
		self.pillars.push( pillar::Pillar::new(p, r) );
	}

	fn update_points(&mut self, dt: f32) {
		let closest = self.get_closest_pillar_to_player();
		if closest.1 <= POINT_DIST_THRESHOLD {
			self.score += self.get_points_from_dist(dt, closest.1);
		}
	}

	#[inline]
	fn get_points_from_dist(&mut self, dt: f32, dist: f32) -> u32 {    // Gets the points scored from the distance to the peg
		(dt * (POINT_DIST_THRESHOLD - dist) * MAX_POINTS_PER_FRAME as f32).ceil() as u32
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
}

fn main() {
	let rl = raylib::init()
			.size(1000, 800)
			.title("Drift")
			.msaa_4x()
			.build();

	rl.set_target_fps(144 * 2);

	let mut g = Game::new(car::Car::new(Vector2 { x: 300.0, y: 300.0 }), rl.load_texture("textures/car/car_body.png"));
	g.add_pillar(Vector2 { x: 300.0 , y: 400.0 }, 7.0);
	g.add_pillar(Vector2 { x: 700.0 , y: 400.0 }, 7.0);
	g.add_pillar(Vector2 { x: 500.0 , y: 300.0 }, 7.0);

	while !rl.window_should_close() {
		g.update(&rl, rl.get_frame_time());

		rl.begin_drawing();
		rl.clear_background(BG_COLOR);
		g.draw(&rl);

		rl.draw_fps(10, 10);
		rl.end_drawing();
	}
}

