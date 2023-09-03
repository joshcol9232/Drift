use raylib::{math::Vector2, drawing::{RaylibDraw, RaylibDrawHandle}, RaylibHandle, color::Color};

use crate::{RED_1, BG_COLOR};

const DEF_PILLAR_RADIUS: f32 = 7.0;

pub struct Pillar {    // Pillars for the player to drift around
	pub pos: Vector2,
	pub radius: f32,
	pub progress: f32, // Progress completed (in radians)
	pub player_start_angle: f32,	// Start angle around pillar (so progress can be calculated)
	pub done: bool,	   // If player has done full 360 around it yet.
}

impl Default for Pillar {
	fn default() -> Pillar {
		Pillar {
			pos: Vector2 { x: 300.0, y: 400.0 },
			radius: DEF_PILLAR_RADIUS,
			progress: 0.0,
			player_start_angle: 0.0,
			done: false,
		}
	}
}

impl Pillar {
	pub fn new(p: Vector2, r: f32) -> Pillar {
		Pillar {
			pos: p,
			radius: r,
			..Default::default()
		}
	}

	pub fn draw(&self, d: &mut RaylibDrawHandle) {
		d.draw_circle_v(self.pos, self.radius, RED_1);
		let col = match self.done {
			true => Color::LIME,
			false => BG_COLOR,
		};
		d.draw_circle_v(self.pos, self.radius - 2.0, col);   // Leaves red circle with line thickness of 2
	}

	#[inline]
	pub fn distance_to(&self, point: Vector2) -> f32 {
		point.distance_to(self.pos)
	}
}
