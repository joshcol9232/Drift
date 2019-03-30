use raylib::{Vector2, RaylibHandle, Rectangle, Color};
use raylib::consts;

use crate::misc;
use crate::RED_1;

const CAR_ACC: f32 = 500.0;
const CAR_W: f32 = 23.0;
const CAR_H: f32 = 40.0;
const HALF_CAR_W: f32 = CAR_W/2.0;
const HALF_CAR_H: f32 = CAR_H/2.0;
const CAR_TURN_SPD: f32 = consts::PI as f32/1.2;
const CAR_RESISTANCE: f32 = 2.718;

pub struct Car {
	pos: Vector2,
	vel: Vector2,
	angle: f32,
	perp: f32   // How perpendicular the car is to it's velocity
}

impl Car {
	pub fn new(p: Vector2) -> Car {
		Car {
			pos: p,
			vel: Vector2::zero(),
			angle: consts::PI as f32,
			perp: 0.0
		}
	}

	pub fn draw(&self, rl: &RaylibHandle) {
		rl.draw_rectangle_pro(Rectangle {
								x: self.pos.x,
								y: self.pos.y,
								width: CAR_W,
								height: CAR_H
							  },
							  Vector2 { x: HALF_CAR_W, y: HALF_CAR_H },
							  -self.angle * consts::RAD2DEG as f32,
							  RED_1);
	}

	pub fn update(&mut self, rl: &RaylibHandle, dt: f32) {
		if rl.is_key_down(consts::KEY_W as i32) {
			self.accelerate(dt, 1.0);
		}
		if rl.is_key_down(consts::KEY_S as i32) {
			self.accelerate(dt, -1.0);
		}
		if rl.is_key_down(consts::KEY_A as i32) {
			self.turn(dt, 1.0);
		}
		if rl.is_key_down(consts::KEY_D as i32) {
			self.turn(dt, -1.0);
		}

		let vel_len = self.vel.length();
		if vel_len > 0.0 {
			self.apply_resistance(dt);
			if vel_len < 0.01 {
				self.vel = Vector2::zero();
			}
		}
		
		self.pos = self.pos + self.vel.scale_by(dt);
	}

	fn accelerate(&mut self, dt: f32, power: f32) {
		let dv = dt * power * CAR_ACC;
		self.vel += misc::get_components(dv, self.angle);
	}

	fn turn(&mut self, dt: f32, amount: f32) {
		self.angle += dt * amount * (CAR_TURN_SPD * (1.5 - self.perp));
	}

	fn apply_resistance(&mut self, dt: f32) {
		self.perp = self.get_perp_value();
		self.vel.scale(CAR_RESISTANCE.powf(-dt * (self.perp + 1.0)));
	}

	#[inline(always)]
	fn get_perp_value(&self) -> f32 {
		1.0 - (self.vel/self.vel.length()).dot(Vector2 { x: self.angle.sin(), y: self.angle.cos() }).abs()
	}
}
