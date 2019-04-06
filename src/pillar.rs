use raylib::{Vector2, RaylibHandle, Color};

use crate::{RED_1, BG_COLOR};

const DEF_PILLAR_RADIUS: f32 = 7.0;

pub struct Pillar {    // Pillars for the player to drift around
	pub pos: Vector2,
	pub radius: f32
}

impl Default for Pillar {
	fn default() -> Pillar {
		Pillar {
			pos: Vector2 { x: 300.0, y: 400.0 },
			radius: DEF_PILLAR_RADIUS
		}
	}
}

impl Pillar {
	pub fn new(p: Vector2, r: f32) -> Pillar {
		Pillar {
			pos: p,
			radius: r
		}
	}

	pub fn draw(&self, rl: &RaylibHandle) {
		rl.draw_circle_v(self.pos, self.radius, RED_1);
		rl.draw_circle_v(self.pos, self.radius - 2.0, BG_COLOR);   // Leaves red circle with line thickness of 2
	}

	#[inline(always)]
	pub fn distance_to(&self, point: Vector2) -> f32 {
		point.distance_to(self.pos)
	}


}
