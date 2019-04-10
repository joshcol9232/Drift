use raylib::{consts, Vector2, RaylibHandle, Rectangle, Color, Texture2D};

use crate::misc;

const CAR_ACC: f32 = 500.0;

pub const CAR_W: f32 = 36.0;
pub const CAR_H: f32 = 56.0;
pub const HALF_CAR_W: f32 = CAR_W/2.0;
pub const HALF_CAR_H: f32 = CAR_H/2.0;
pub const TRAIL_DRAW_W: f32 = HALF_CAR_W - 2.0;   // Where trails are drawn relative to center
pub const TRAIL_DRAW_H: f32 = HALF_CAR_H - 6.0;
pub const COM_OFF: f32 = 8.0; // Centre of mass

const CAR_TURN_SPD: f32 = 7.0 * consts::PI as f32;
const CAR_RESISTANCE: f32 = 2.718;
const HALF_PI: f32 = (consts::PI/2.0) as f32;

pub struct Car {
	pub pos: Vector2,
	vel: Vector2,
	pub vel_mag: f32,
	pub throttle: f32,
	pub angle: f32,
	pub angular_vel: f32,
	angular_acc: f32,
	pub perp: f32,   // How perpendicular the car is to it's velocity
	pub drifting: bool
}

impl Car {
	pub fn new(p: Vector2) -> Car {
		Car {
			pos: p,
			vel: Vector2::zero(),
			vel_mag: 0.0,
			throttle: 0.0,
			angle: consts::PI as f32,
			angular_vel: 0.0,
			angular_acc: 0.0,
			perp: 0.0,
			drifting: false
		}
	}

	pub fn reset(&mut self) {
		self.pos = Vector2 { x: 100.0, y: 600.0 };
		self.vel = Vector2::zero();
		self.throttle = 0.0;
		self.angle = consts::PI as f32;
		self.angular_vel = 0.0;
		self.angular_acc = 0.0;
	}

	pub fn draw(&self, texture: &Texture2D, rl: &RaylibHandle) {
        rl.draw_texture_pro(texture,
                            Rectangle {
                                x: 0.0,
                                y: 0.0,
                                width: CAR_W,
                                height: CAR_H
                            },
                            Rectangle {
                                x: self.pos.x,
                                y: self.pos.y,
                                width: CAR_W,
                                height: CAR_H
                            },
                            Vector2 {
                                x: HALF_CAR_W,
                                y: HALF_CAR_H + COM_OFF
                            },
                            -self.angle * consts::RAD2DEG as f32,
                            Color::WHITE
                            );
	}

	pub fn update(&mut self, rl: &RaylibHandle, dt: f32) {
		let up_key = rl.is_key_down(consts::KEY_W as i32);
		let down_key = rl.is_key_down(consts::KEY_S as i32);
		if up_key || down_key {
			if up_key {
				self.throttle = 1.0;
			}
			if down_key {
				self.throttle = -1.0;
			}

			self.accelerate(dt, self.throttle);
		} else {
			self.throttle = 0.0;
		}


		self.vel_mag = self.vel.length();
		self.angular_acc = 0.0;

		if self.vel_mag > 0.0 {
			self.perp = self.get_perp_value();

			self.drifting = self.perp.abs() > 0.35 && self.vel_mag > 10.0;

			self.apply_resistance(dt);

			if rl.is_key_down(consts::KEY_A as i32) {
				self.angular_acc = (self.vel_mag/200.0).min(1.0);
				self.turn(dt, self.angular_acc);
			}
			if rl.is_key_down(consts::KEY_D as i32) {
				self.angular_acc = -(self.vel_mag/200.0).min(1.0);
				self.turn(dt, self.angular_acc);
			}

			if self.vel_mag < 0.2 {
				self.vel = Vector2::zero();
			}
		}

/*
 *      if rl.is_gamepad_available(consts::GAMEPAD_PLAYER1 as i32) {
 *         self.accelerate(dt, ((rl.get_gamepad_axis_movement(consts::GAMEPAD_PLAYER1 as i32, consts::GAMEPAD_XBOX_AXIS_RT as i32)) + 1.0)/2.0);
 *         self.accelerate(dt, -((rl.get_gamepad_axis_movement(consts::GAMEPAD_PLAYER1 as i32, 2)) + 1.0)/2.0);  // Brake
 *
 *
 *         let turn_amount = rl.get_gamepad_axis_movement(consts::GAMEPAD_PLAYER1 as i32, consts::GAMEPAD_XBOX_AXIS_LEFT_X as i32);
 *         if turn_amount.abs() > 0.1 {
 *            self.angular_acc = (self.vel_mag/200.0).min(1.0) * -turn_amount;
 *            self.turn(dt, self.angular_acc);
 *         }
 *      }
 */

		self.angle += self.angular_vel * dt;
		
		self.pos = self.pos + self.vel.scale_by(dt);
	}

	fn accelerate(&mut self, dt: f32, power: f32) {
		let dv = dt * power * CAR_ACC;
		self.vel += misc::get_components(dv, self.angle);
	}

	fn turn(&mut self, dt: f32, amount: f32) {
		self.angular_vel += dt * amount * CAR_TURN_SPD;
	}

	fn apply_resistance(&mut self, dt: f32) {
		self.angular_vel *= (100.0 as f32).powf(-dt * (2.0 - self.perp.abs()));

		let d_hor_v = -self.perp * dt * 500.0;
		let ang = self.angle + HALF_PI; // Angle perpendicular to car to apply resistive vel on

		self.vel += Vector2 { x: d_hor_v * ang.sin(), y: d_hor_v * ang.cos() };
		self.vel.scale(CAR_RESISTANCE.powf(-dt));   // All deceleration
	}

	fn get_perp_value(&self) -> f32 {
		let ang = self.angle + HALF_PI;
		(self.vel/self.vel_mag).dot(Vector2 { x: ang.sin(), y: ang.cos() })
	}
}
