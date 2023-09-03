extern crate raylib;
extern crate rand;
extern crate rayon;

mod traits;
mod car;
mod drift_trail;
mod dust_system;
mod pillar;
mod misc;

use raylib::{color::Color, math::Vector2, drawing::{RaylibDraw, RaylibDrawHandle}, RaylibHandle, RaylibThread, consts};
use crate::{
	traits::*,
};

static BG_COLOR: Color = Color { r: 230, g: 230, b: 220, a: 255 };
static RED_1: Color = Color { r: 190, g: 36, b: 25, a: 255 };
static RED_2: Color = Color { r: 232, g: 89, b: 79, a: 255 };
static CHARCOAL: Color = Color { r: 38, g: 38, b: 38, a: 255 };

const POINT_DIST_THRESHOLD: f32 = 200.0; //100.0;
const MAX_POINTS_PER_FRAME: u32 = 5;
const TWO_PI: f32 = consts::PI as f32 * 2.0;

struct Game {
	player: car::Car,
	pillars: Vec<pillar::Pillar>,
	closest_pillar_to_player: (i32, f32),
	player_is_scoring_points: bool,
	score: u32,
	use_debug: bool,
}

impl Game {
	fn new(rl: &mut RaylibHandle, rl_thread: &RaylibThread, player_pos: Vector2) -> Game {
		Game {
			player: car::Car::new(rl, rl_thread, player_pos),
			pillars: vec![],
			closest_pillar_to_player: (0, -1.0),
			player_is_scoring_points: false,
			score: 0,
			use_debug: true,
		}
	}

	fn draw(&mut self, rl: &mut RaylibHandle, rl_thread: &RaylibThread) {
        let time = rl.get_time();
        let mut d = rl.begin_drawing(&rl_thread);
        d.clear_background(BG_COLOR);

		// draw trails below stuff
		self.player.draw_trails(&mut d, time);

		for p in self.pillars.iter() {
			p.draw(&mut d);
		}

		self.player.draw(&mut d);

		if self.use_debug {
			let closest_pillar_pos = self.pillars[self.closest_pillar_to_player.0 as usize].pos;
			if self.player_is_scoring_points {
				d.draw_line_ex(closest_pillar_pos, self.player.pos, 2.0, Color::BLUE);
			}
			d.draw_circle_v(closest_pillar_pos, POINT_DIST_THRESHOLD, Color::new(0, 100, 0, 100));

			d.draw_text(format!("Trail nodes: {}", self.player.get_trail_node_count()).as_str(), 10, 32, 20, CHARCOAL);
			d.draw_text(format!("Player speed: {:.1}", self.player.vel_mag).as_str(), 10, 54, 20, CHARCOAL);
			d.draw_text(format!("Player perp: {:.3}", self.player.perp).as_str(), 10, 76, 20, CHARCOAL);
			d.draw_text(format!("Particle count: {}", self.player.get_particle_count()).as_str(), 10, 120, 20, CHARCOAL);
		}
		d.draw_text(format!("Score: {}", self.score).as_str(), 400, 10, 20, RED_2);

        d.draw_fps(10, 10);
	}

	fn update(&mut self, dt: f32, rl: &mut RaylibHandle) {
		self.player.update(&rl, dt);

		self.closest_pillar_to_player = self.get_closest_pillar_to_player();

		// Player has to do full 360 around pillar before moving on.
		let mut pillar = &mut self.pillars[self.closest_pillar_to_player.0 as usize];
		if !pillar.done && self.closest_pillar_to_player.1 <= POINT_DIST_THRESHOLD {//self.player.drifting && !self.pillars[self.closest_pillar_to_player.0 as usize].done && self.closest_pillar_to_player.1 <= POINT_DIST_THRESHOLD {
			let curr_angle = pillar.pos.angle_to(self.player.pos);
			if self.player_is_scoring_points {  // If already scoring points, then check for full 360
				let angle_diff = misc::get_angle_diff(pillar.player_start_angle, curr_angle).abs();
				println!("Start angle: {}, Curr angle: {}, diff {}", pillar.player_start_angle, curr_angle, angle_diff);

				pillar.progress = angle_diff/TWO_PI;
				println!("Pillar progress: {}", pillar.progress);
				
				/*
				if pillar.progress >= 0.99 {
					pillar.done = true
				}
				*/
			} else {
				pillar.player_start_angle = curr_angle;
				self.player_is_scoring_points = true;
			}
			self.score += self.get_points_from_dist(dt, self.closest_pillar_to_player.1);
		} else if self.player_is_scoring_points {
			pillar.progress = 0.0;
			self.player_is_scoring_points = false;
		}

		if rl.is_key_pressed(consts::KeyboardKey::KEY_R) { self.reload() }
		if rl.is_key_pressed(consts::KeyboardKey::KEY_F10) { self.use_debug = !self.use_debug }
	}

	fn reload(&mut self) {
		self.player.reset();
		self.score = 0;
	}

	#[inline]
	fn add_pillar(&mut self, p: Vector2, r: f32) {
		self.pillars.push( pillar::Pillar::new(p, r) );
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
	let (mut rl, rl_thread) = raylib::init()
		.size(1000, 800)
		.title("Drift")
		.msaa_4x()
		.build();

	rl.set_target_fps(144 * 2);

	let mut g = Game::new(&mut rl, &rl_thread, Vector2::new(300.0, 300.0));
	/*
	g.add_pillar(Vector2::new(300.0, 400.0), 7.0);
	g.add_pillar(Vector2::new(700.0, 400.0), 7.0);
	g.add_pillar(Vector2::new(500.0, 300.0), 7.0);
	*/

	g.add_pillar(Vector2::new(500.0, 400.0), 7.0);

    while !rl.window_should_close() {
        g.update(rl.get_frame_time(), &mut rl);
        g.draw(&mut rl, &rl_thread);
    }
}

