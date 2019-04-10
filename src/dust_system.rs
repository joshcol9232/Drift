use raylib::{Vector2, RaylibHandle, consts::PI};
use rand::Rng;

use crate::{CHARCOAL, DRIFT_TRAIL_WIDTH, misc};

const DUST_PARTICLE_MAX_LIFESPAN: f64 = 2.0;  // In seconds
const DUST_PARTICLE_MIN_LIFESPAN: f64 = 1.0;  // In seconds

pub const DUST_PARTICLES_EMM_RATE: f32 = 4.0; // How many emitted per sec, for every pixel per second the player is moving at, and the acceleration multiplier
const DUST_PARTICLES_EMM_RADIUS: f32 = DRIFT_TRAIL_WIDTH;

const DUST_PARTICLE_MIN_SPEED: f32 = 10.0;
const DUST_PARTICLE_MAX_SPEED: f32 = 30.0;

const DUST_PARTICLE_ANGULAR_VARIATION: f32 = PI as f32 * 2.0;

const DUST_PARTICLE_RADIUS: f32 = 8.0;

const DUST_EMMISION_TIME: f32 = 0.01;


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
			radius: DUST_PARTICLE_RADIUS,
			time_created: 0.0,
			lifespan: 1.0,
			alpha: 255
		}
	}
}

impl Particle {
	fn new(p: Vector2, v: Vector2, time: f64, life: f64) -> Particle {
		Particle {
			pos: p,
			vel: v,
			time_created: time,
			lifespan: life,
			..Default::default()
		}
	}

	fn update(&mut self, dt: f32, time: f64) {
		let norm_life: f32 = (1.0 - (time - self.time_created)/self.lifespan) as f32;
		self.alpha = (norm_life.powi(3) * 230.0).min(230.0).ceil() as u8;
		self.radius = DUST_PARTICLE_RADIUS + (2.0 * DUST_PARTICLE_RADIUS * (1.0 - norm_life));
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
	particle_emm_rate: f32,  // In p per second
	emit: bool,
	pub finished: bool,
	time_emitting: f32,
	spawn_pos: Vector2
}

impl ParticleSystem {
	pub fn new(p: Vector2, rate: f32) -> ParticleSystem {
		ParticleSystem {
			particles: vec![],
			particle_emm_rate: rate,
			emit: true,
			finished: false,
			time_emitting: 0.0,
			spawn_pos: p
		}
	}

	pub fn update(&mut self, dt: f32, time: f64) {
		self.kill_particles(time);
		if self.emit {
			self.spawn_particles(dt, time);
			self.time_emitting += dt;
			self.emit = self.time_emitting <= DUST_EMMISION_TIME;
		} else {
			self.finished = self.particles.len() == 0;
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
	pub fn get_particle_count(&self) -> usize {
		self.particles.len()
	}

	fn spawn_particles(&mut self, dt: f32, time: f64) {
		let p_num = (self.particle_emm_rate * dt).floor() as i32;
		for _ in 0..p_num {
			self.spawn_single_particle(time);
		}
	}

	fn spawn_single_particle(&mut self, time: f64) {
		let mut random_thread = rand::thread_rng();

		let vel = misc::get_components(random_thread.gen_range(DUST_PARTICLE_MIN_SPEED, DUST_PARTICLE_MAX_SPEED),                   // Random speed
												 random_thread.gen_range(-DUST_PARTICLE_ANGULAR_VARIATION, DUST_PARTICLE_ANGULAR_VARIATION)); // Random angle

		let pos = self.spawn_pos + Vector2 { x: random_thread.gen_range(0.0, DUST_PARTICLES_EMM_RADIUS), y: random_thread.gen_range(0.0, DUST_PARTICLES_EMM_RADIUS) };

		self.particles.push( Particle::new(pos, vel, time, random_thread.gen_range(DUST_PARTICLE_MIN_LIFESPAN, DUST_PARTICLE_MAX_LIFESPAN)) );
	}

	#[inline]
	fn kill_particles(&mut self, time: f64) {
		self.particles.retain(|i|time - i.time_created <= i.lifespan);
	}
}
