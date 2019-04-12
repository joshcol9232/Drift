use raylib::{Vector2, RaylibHandle, consts::PI};
use rand::Rng;

use crate::{CHARCOAL, misc::get_components};

const DUST_PARTICLE_MAX_RAD: f32 = 10.0;  // Starting radius
const DUST_PARTICLE_MIN_RAD: f32 = 1.0;
const DUST_PARTICLE_EXPANSION_RATE: f32 = 15.0;  // Pixels per second increase of radius

const DUST_PARTICLE_MAX_LIFESPAN: f64 = 0.15;  // In seconds, per each 1 pixel of radius
const DUST_PARTICLE_MIN_LIFESPAN: f64 = 0.08;  // In seconds

pub const DUST_PARTICLES_EMM_RATE: f32 = 300.0; // How many emitted per sec, for every pixel per second the player is moving at, and the acceleration multiplier

const DUST_PARTICLE_MIN_SPEED: f32 = 10.0;
const DUST_PARTICLE_MAX_SPEED: f32 = 100.0;

const DUST_PARTICLE_ANGULAR_VARIATION: f32 = PI as f32/4.0;


struct Particle {
	pos: Vector2,
	vel: Vector2,
	radius: f32,
	time_created: f64,
	lifespan: f64,
	alpha: u8
}

impl Default for Particle {
	fn default() -> Particle {
		Particle {
			pos: Vector2::zero(),
			vel: Vector2::zero(),
			radius: 8.0,
			time_created: 0.0,
			lifespan: 1.0,
			alpha: 255
		}
	}
}

impl Particle {
	fn new(p: Vector2, v: Vector2, time: f64, life: f64, rad: f32) -> Particle {
		Particle {
			pos: p,
			vel: v,
			radius: rad,
			time_created: time,
			lifespan: life * rad as f64,
			..Default::default()
		}
	}

	fn update(&mut self, dt: f32, time: f64) {
		let norm_life: f32 = (1.0 - (time - self.time_created)/self.lifespan) as f32;
		self.alpha = (norm_life.powi(2) * 230.0).ceil() as u8;
		self.radius += dt * DUST_PARTICLE_EXPANSION_RATE;
		self.pos += self.vel.scale_by(dt);
	}

	fn draw(&self, rl: &RaylibHandle) {
		let mut col = CHARCOAL;
		col.a = self.alpha;
		rl.draw_circle_v(self.pos, self.radius, col);
	}
}

pub struct ParticleSystem {
	particles: Vec<Particle>,
	pub emit: bool,
	pub max_rad: f32,
	em_rate: f32,
	pub spawn_pos: Vector2,
	pub spawn_angle: f32,
	spawn_timer: f32,
	rand_thread: rand::rngs::ThreadRng
}

impl Default for ParticleSystem {
	fn default() -> ParticleSystem {
		ParticleSystem {
			particles: vec![],
			emit: false,
			em_rate: DUST_PARTICLES_EMM_RATE,
			max_rad: DUST_PARTICLE_MAX_RAD,
			spawn_pos: Vector2::zero(),
			spawn_angle: 0.0,
			spawn_timer: 0.0,
			rand_thread: rand::thread_rng()
		}
	}
}

impl ParticleSystem {
	pub fn new(p: Vector2, angle: f32) -> ParticleSystem {
		ParticleSystem {
			spawn_pos: p,
			spawn_angle: angle,
			..Default::default()
		}
	}

	pub fn update(&mut self, dt: f32, time: f64) {
		self.spawn_timer += dt;

		self.kill_particles(time);
		if self.emit {
			self.spawn_particles(dt, time);
		}

		for p in self.particles.iter_mut() {
			p.update(dt, time);
		}
	}

	pub fn draw(&self, rl: &RaylibHandle) {
		for p in self.particles.iter() {
			p.draw(rl);
		}
	}

	#[inline]
	fn get_particle_count(&self) -> usize {
		self.particles.len()
	}

	fn spawn_particles(&mut self, dt: f32, time: f64) {
		let p_num = (self.em_rate * dt).floor() as u32;
		if self.spawn_timer >= 1.0/self.em_rate {
			self.spawn_timer = 0.0;
			self.spawn_single_particle(time);
		}

		for _ in 0..p_num {
			self.spawn_single_particle(time);
		}
	}

	fn spawn_single_particle(&mut self, time: f64) {
		let vel = get_components(self.rand_thread.gen_range(DUST_PARTICLE_MIN_SPEED, DUST_PARTICLE_MAX_SPEED),                   // Random speed
										 self.spawn_angle + self.rand_thread.gen_range(-DUST_PARTICLE_ANGULAR_VARIATION, DUST_PARTICLE_ANGULAR_VARIATION)); // Random angle

		let mut rad = DUST_PARTICLE_MIN_RAD;
		if rad < self.max_rad {
			rad = self.rand_thread.gen_range(DUST_PARTICLE_MIN_RAD, self.max_rad);
		}

		self.particles.push( Particle::new(self.spawn_pos, vel, time,
													  self.rand_thread.gen_range(DUST_PARTICLE_MIN_LIFESPAN, DUST_PARTICLE_MAX_LIFESPAN),
													  rad) );
	}

	#[inline]
	fn kill_particles(&mut self, time: f64) {
		self.particles.retain(|i|time - i.time_created <= i.lifespan);
	}
}

pub struct CarDustSystems {
	pub left: ParticleSystem,
	pub right: ParticleSystem
}

impl CarDustSystems {
	#[inline]
	pub fn update(&mut self, dt: f32, time: f64) {
		self.left.update(dt, time);
		self.right.update(dt, time);
	}

	#[inline]
	pub fn draw(&self, rl: &RaylibHandle) {
		self.left.draw(rl);
		self.right.draw(rl);
	}

	pub fn emit(&mut self, dt: f32, time: f64, player_ang: f32, rate_multiplier: f32, back_left_pos: Vector2, back_right_pos: Vector2) {
		self.left.emit = true;
		self.right.emit = true;

		self.left.em_rate = DUST_PARTICLES_EMM_RATE * rate_multiplier;
		self.right.em_rate = self.left.em_rate;

		self.left.max_rad = (DUST_PARTICLE_MAX_RAD * rate_multiplier).max(0.5);
		self.right.max_rad = self.left.max_rad;

		self.left.spawn_pos = back_left_pos;
		self.right.spawn_pos = back_right_pos;

		self.left.spawn_angle = player_ang + PI as f32;
		self.right.spawn_angle = self.left.spawn_angle;

		self.left.spawn_particles(dt, time);
		self.right.spawn_particles(dt, time);
	}

	#[inline]
	pub fn get_particle_count(&self) -> usize {
		self.left.get_particle_count() + self.right.get_particle_count()
	}
}

impl Default for CarDustSystems {
	fn default() -> CarDustSystems {
		CarDustSystems {
			left: ParticleSystem::new(Vector2::zero(), 0.0),
			right: ParticleSystem::new(Vector2::zero(), 0.0)
		}
	}
}

