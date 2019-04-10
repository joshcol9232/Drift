use raylib::{Vector2, RaylibHandle, consts::PI};
use rand::Rng;

use crate::{CHARCOAL, DRIFT_TRAIL_WIDTH, misc};

const DUST_PARTICLE_MAX_LIFESPAN: f64 = 0.5;  // In seconds
const DUST_PARTICLE_MIN_LIFESPAN: f64 = 0.4;  // In seconds

const DUST_PARTICLES_EMM_RATE: f32 = 1000.0; // How many emitted per sec
const DUST_PARTICLES_EMM_RADIUS: f32 = DRIFT_TRAIL_WIDTH;

const DUST_PARTICLE_MIN_SPEED: f32 = 10.0;
const DUST_PARTICLE_MAX_SPEED: f32 = 100.0;
const DUST_PARTICLE_DECLERATION: f32 = 1.0;

const DUST_PARTICLE_ANGULAR_VARIATION: f32 = PI as f32 * 2.0;

const DUST_PARTICLE_RADIUS: f32 = 5.0;


struct Particle {
	pos: Vector2,
	vel: Vector2,
	time_created: f64,
	lifespan: f64,
	alpha: u8
}

impl Default for Particle {
	fn default() -> Particle {
		Particle {
			pos: Vector2::zero(),
			vel: Vector2::zero(),
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
		self.alpha = ((1.0 - ((time - self.time_created)/self.lifespan)) * 255.0).min(255.0).ceil() as u8;

		//self.vel.scale(dt * DUST_PARTICLE_DECLERATION);
		self.pos += self.vel.scale_by(dt);
	}

	#[inline]
	fn draw(&self, rl: &RaylibHandle) {
		let mut col = CHARCOAL;
		col.a = self.alpha;
		rl.draw_circle_v(self.pos, DUST_PARTICLE_RADIUS, col);
	}
}

pub struct ParticleSystem {
	particles: Vec<Particle>,
	particle_emm_rate: f32,  // In p per second
	spawn_pos: Vector2
}

impl ParticleSystem {
	pub fn new(p: Vector2) -> ParticleSystem {
		ParticleSystem {
			particles: vec![],
			particle_emm_rate: DUST_PARTICLES_EMM_RATE,
			spawn_pos: p
		}
	}

	pub fn update(&mut self, dt: f32, time: f64) {
		self.kill_particles(time);
		self.spawn_particles(dt, time);

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
