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
static DRIFT_SMOKE: Color = Color { r: 110, g: 110, b: 110, a: 255 };

const POINT_DIST_THRESHOLD: f32 = 100.0;
const DRIFT_TRAIL_WIDTH: f32 = 3.5;
const TRAIL_DURATION: f64 = 3.5; // In seconds
const MAX_POINTS_PER_FRAME: u32 = 5;
const TRAIL_PLACEMENT_INTERVAL: f32 = 0.007;  // Place a trail every x seconds.
const DUST_SOURCE_PLACEMENT_INTERVAL: u8 = 5;  // Place a dust source every x trails are put down

struct Game {
	player: car::Car,
	pillars: Vec<pillar::Pillar>,
	trail_nodes: Vec<drift_trail::DriftTrailSet>,
	particle_systems: Vec<dust_system::ParticleSystem>,
   car_texture: Texture2D,
   trail_timer: f32,
	dust_trail_count: u8,
	score: u32
}

impl<'a> Game {
	pub fn new(p: car::Car, c_texture: Texture2D) -> Game {
		Game {
			player: p,
			pillars: vec![],
			trail_nodes: vec![],
			particle_systems: vec![],
			car_texture: c_texture,
			trail_timer: 0.0,
			dust_trail_count: 0,
			score: 0
		}
	}

	pub fn draw(&self, rl: &RaylibHandle) {
		self.draw_trails(rl, rl.get_time());

		for p in self.pillars.iter() {
			p.draw(rl);
		}

		for ps in self.particle_systems.iter() {
			ps.draw(rl);
		}

		self.player.draw(&self.car_texture, rl);

		rl.draw_text(format!("Score: {}", self.score).as_str(), 400, 10, 20, RED_2);
		rl.draw_text(format!("Trail nodes: {}", self.trail_nodes.len()).as_str(), 10, 32, 20, CHARCOAL);
		rl.draw_text(format!("Player speed: {:.1}", self.player.vel_mag).as_str(), 10, 54, 20, CHARCOAL);
		rl.draw_text(format!("Player perp: {:.3}", self.player.perp).as_str(), 10, 76, 20, CHARCOAL);
		rl.draw_text(format!("Prt system count: {}", self.particle_systems.len()).as_str(), 10, 98, 20, CHARCOAL);
		rl.draw_text(format!("Particle count: {}", self.get_particle_count()).as_str(), 10, 120, 20, CHARCOAL);
	}

	pub fn update(&mut self, rl: &RaylibHandle, dt: f32) {
		self.trail_timer += dt;
		let curr_time = rl.get_time();

		self.kill_finished_particle_systems();
		self.kill_dead_trail_nodes(curr_time);
		self.player.update(rl, dt);

		for ps in self.particle_systems.iter_mut() {
			ps.update(dt, curr_time);
		}

		if self.player.drifting {
			self.place_trails(curr_time);
			self.place_dust_sources();
			self.update_points(dt);
		}

		if rl.is_key_pressed(consts::KEY_R as i32) { self.reload() }
	}

	fn reload(&mut self) {
		self.player.reset();
		self.trail_nodes = vec![];
		self.score = 0;
	}

	pub fn add_pillar(&mut self, p: Vector2, r: f32) {
		self.pillars.push( pillar::Pillar::new(p, r) );
	}

	pub fn add_dust_source(&mut self, pos: Vector2) {
		self.particle_systems.push( dust_system::ParticleSystem::new(pos, dust_system::DUST_PARTICLES_EMM_RATE * self.player.perp.abs() * self.player.throttle.abs() * self.player.vel_mag.abs()) );
	}

	fn get_particle_count(&self) -> usize {
		let mut total: usize = 0;
		for ps in self.particle_systems.iter() {
			total += ps.get_particle_count();
		}
		total
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

	fn kill_dead_trail_nodes(&mut self, time: f64) {
		self.trail_nodes.retain(|i|time - i.time_created <= TRAIL_DURATION);
	}

	fn kill_finished_particle_systems(&mut self) {
		self.particle_systems.retain(|i|!i.finished);
	}

	fn place_trails(&mut self, time: f64) {
		if self.trail_timer >= TRAIL_PLACEMENT_INTERVAL {
			self.trail_nodes.push(drift_trail::DriftTrailSet::new(self.player.pos, car::TRAIL_DRAW_W, car::TRAIL_DRAW_H, -self.player.angle, time));
			self.trail_timer = 0.0;
			self.dust_trail_count += 1;
		}
	}

	fn place_dust_sources(&mut self) {
		if self.dust_trail_count >= DUST_SOURCE_PLACEMENT_INTERVAL {
			self.add_dust_source(self.player.pos + misc::rotate_vec(Vector2 { x: -drift_trail::BACK_WHEEL_X_OFF + car::HALF_CAR_W - 5.0, y: drift_trail::BACK_WHEEL_Y_OFF - car::HALF_CAR_H }, -self.player.angle));  // Left wheel
			self.add_dust_source(self.player.pos + misc::rotate_vec(Vector2 { x: drift_trail::BACK_WHEEL_X_OFF - car::HALF_CAR_W + 5.0, y: drift_trail::BACK_WHEEL_Y_OFF - car::HALF_CAR_H }, -self.player.angle));  // Right wheel
			self.dust_trail_count = 0;
		}
	}

	fn draw_trails(&self, rl: &RaylibHandle, time: f64) {
		let mut last: &drift_trail::DriftTrailSet = &drift_trail::DriftTrailSet::default();
		for (i, t) in self.trail_nodes.iter().enumerate() {
			if i > 0 && last.left_front.distance_to(t.left_front) < 10.0 {
				let mut col = CHARCOAL;
				col.a = ((3.0 * ((t.time_created - time)/TRAIL_DURATION) + 4.0).log2() * 255.0).min(255.0) as u8;  // Alpha value for this line
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

