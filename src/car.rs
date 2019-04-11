use raylib::{consts, Vector2, RaylibHandle, Rectangle, Color, Texture2D};

use crate::{misc, drift_trail, dust_system, CHARCOAL};

const CAR_ACC: f32 = 500.0;

pub const CAR_W: f32 = 36.0;
pub const CAR_H: f32 = 56.0;
pub const HALF_CAR_W: f32 = CAR_W/2.0;
pub const HALF_CAR_H: f32 = CAR_H/2.0;
pub const TRAIL_DRAW_W: f32 = HALF_CAR_W - 2.0;   // Where trails are drawn relative to center
pub const TRAIL_DRAW_H: f32 = HALF_CAR_H - 6.0;
pub const COM_OFF: f32 = 8.0; // Centre of mass
const BACK_WHEEL_X_OFF: f32 = 5.0;
const BACK_WHEEL_Y_OFF: f32 = 15.0;
const FRONT_WHEEL_X_OFF: f32 = 5.0;
const FRONT_WHEEL_Y_OFF: f32 = 8.0;

const CAR_TURN_SPD: f32 = 7.0 * consts::PI as f32;
const CAR_RESISTANCE: f32 = 2.718;
const HALF_PI: f32 = (consts::PI/2.0) as f32;
const TRAIL_DURATION: f64 = 3.5; // In seconds
const TRAIL_PLACEMENT_INTERVAL: f32 = 0.007;  // Place a trail every x seconds.
pub const DRIFT_TRAIL_WIDTH: f32 = 3.5;


pub struct Car {
	pub pos: Vector2,
	vel: Vector2,
	pub vel_mag: f32,
	pub throttle: f32,
	pub angle: f32,
	pub angular_vel: f32,
	angular_acc: f32,
	pub perp: f32,   // How perpendicular the car is to it's velocity
	pub drifting: bool,

	trail_nodes: Vec<drift_trail::DriftTrailSet>,
	dust_sys: dust_system::CarDustSystems,
   trail_timer: f32,
}

impl Default for Car {
	fn default() -> Car {
		Car {
			pos: Vector2::zero(),
			vel: Vector2::zero(),
			vel_mag: 0.0,
			throttle: 0.0,
			angle: consts::PI as f32,
			angular_vel: 0.0,
			angular_acc: 0.0,
			perp: 0.0,
			drifting: false,

			trail_nodes: vec![],
			dust_sys: dust_system::CarDustSystems::default(),
			trail_timer: 0.0,
		}
	}
}

impl Car {
	pub fn new(p: Vector2) -> Car {
		Car {
			pos: p,
			..Default::default()
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
		self.draw_trails(rl, rl.get_time());

		self.dust_sys.draw(rl);

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
		let curr_time = rl.get_time();
		self.trail_timer += dt;

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

		self.kill_dead_trail_nodes(curr_time);

		self.dust_sys.update(dt, curr_time);

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

			if self.drifting {
				let wheel_positions: [Vector2; 4] = self.get_wheel_positions();
				self.dust_sys.emit(dt, curr_time, self.pos, self.angle, self.perp.abs(), wheel_positions[2], wheel_positions[3]);
				self.place_trails(curr_time, &wheel_positions);
			} else if self.dust_sys.left_back.emit || self.dust_sys.right_back.emit {
				self.dust_sys.left_back.emit = false;
				self.dust_sys.right_back.emit = false;
			}
		}

		if rl.is_gamepad_available(consts::GAMEPAD_PLAYER1 as i32) {   // Stuff for xbox controllers goes in here
			self.throttle = ((rl.get_gamepad_axis_movement(consts::GAMEPAD_PLAYER1 as i32, consts::GAMEPAD_XBOX_AXIS_RT as i32)) + 1.0)/2.0 - ((rl.get_gamepad_axis_movement(consts::GAMEPAD_PLAYER1 as i32, 2)) + 1.0)/2.0;
			self.accelerate(dt, self.throttle);


			let turn_amount = rl.get_gamepad_axis_movement(consts::GAMEPAD_PLAYER1 as i32, consts::GAMEPAD_XBOX_AXIS_LEFT_X as i32);
			if turn_amount.abs() > 0.1 {
				self.angular_acc = (self.vel_mag/200.0).min(1.0) * -turn_amount;
				self.turn(dt, self.angular_acc);
			}
		}

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

	fn get_wheel_positions(&self) -> [Vector2; 4] {
		[misc::rotate_vec(Vector2 { x: -HALF_CAR_W + FRONT_WHEEL_X_OFF, y: HALF_CAR_H - COM_OFF - FRONT_WHEEL_Y_OFF }, -self.angle) + self.pos, // Left front
		 misc::rotate_vec(Vector2 { x: HALF_CAR_W - FRONT_WHEEL_X_OFF, y: HALF_CAR_H - COM_OFF - FRONT_WHEEL_Y_OFF }, -self.angle) + self.pos,  // Right front
		 misc::rotate_vec(Vector2 { x: -HALF_CAR_W + BACK_WHEEL_X_OFF, y: -HALF_CAR_H - COM_OFF + BACK_WHEEL_Y_OFF }, -self.angle) + self.pos,  // Left back
		 misc::rotate_vec(Vector2 { x: HALF_CAR_W - BACK_WHEEL_X_OFF, y: -HALF_CAR_H - COM_OFF + BACK_WHEEL_Y_OFF }, -self.angle) + self.pos]   // Right back
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

	fn place_trails(&mut self, time: f64, wheel_positions: &[Vector2; 4]) {
		if self.trail_timer >= TRAIL_PLACEMENT_INTERVAL {
			self.trail_nodes.push(drift_trail::DriftTrailSet::new(self.pos, time, wheel_positions));
			self.trail_timer = 0.0;
		}
	}

	fn kill_dead_trail_nodes(&mut self, time: f64) {
		self.trail_nodes.retain(|i|time - i.time_created <= TRAIL_DURATION);
	}

	#[inline]
	pub fn get_particle_count(&self) -> usize {
		self.dust_sys.get_particle_count()
	}

	#[inline]
	pub fn get_trail_node_count(&self) -> usize {
		self.trail_nodes.len()
	}
}
