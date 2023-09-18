
use bracket_lib::prelude::*;
use crate::{constants::{OBSTACLE_HOLE_SIZE, SCREEN_HEIGHT, OBSTACLE_WIDTH, KID_X_POSITION}};
use crate::kid::Kid;


#[derive(Clone, Copy)]
pub struct Obstacle {
    pub x: i32,
    pub gap_y: i32,
    size: i32,
    pub is_passed: bool,
}

impl Obstacle {
    pub fn new(x: i32) -> Self {
        Obstacle {
            x,
            gap_y: Self::get_gap_y_random(),
            size: OBSTACLE_HOLE_SIZE,
            is_passed: false,
        }
    }

    pub fn get_gap_y_random() -> i32 {
        let mut random: RandomNumberGenerator = RandomNumberGenerator::new();
        random.range(
            OBSTACLE_HOLE_SIZE / 2 + 2,
            SCREEN_HEIGHT - OBSTACLE_HOLE_SIZE / 2 - 2,
        )
    }

    pub fn render_view(&mut self, ctx: &mut BTerm) {
        for y in 0..self.gap_y - self.size / 2 {
            ctx.set(self.x, y, LIGHT_PINK, BLACK, to_cp437('|'));
            ctx.set(self.x + OBSTACLE_WIDTH, y, LIGHT_PINK, BLACK, to_cp437('|'));
        }

        for y in self.gap_y + self.size / 2..SCREEN_HEIGHT {
            ctx.set(self.x, y, LIGHT_PINK, BLACK, to_cp437('|'));
            ctx.set(self.x + OBSTACLE_WIDTH, y, LIGHT_PINK, BLACK, to_cp437('|'));
        }
        self.x -= 1;
    }

    pub fn hit_obstacle(&self, kid: &Kid) -> bool {
        if KID_X_POSITION + 5 < self.x || KID_X_POSITION + 5 > self.x + OBSTACLE_WIDTH {
            return false;
        }
        if kid.y + 5 < self.gap_y - self.size / 2 || kid.y + 4 > self.gap_y + self.size / 2 {
            return true;
        }
        return false;
    }
}