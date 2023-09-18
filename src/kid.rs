use crate::constants::{DEFAULT_FALL_TIME, SCREEN_HEIGHT, KID_X_POSITION};
use bracket_lib::prelude::*;

#[derive(PartialEq)]
enum KidState {
    Falling,
    Jumping,
}

pub struct Kid {
    pub y: i32,
    state: KidState,
    fall_time: f32,
    jump_time: f32,
    start_falling_y: i32,
    frame_time: f32,
    running_frame: i32,
}

impl Kid {
    pub fn new(y_: i32) -> Self {
        Kid {
            y: y_,
            state: KidState::Falling,
            fall_time: DEFAULT_FALL_TIME,
            jump_time: 0.0,
            start_falling_y: y_,
            frame_time: 0.0,
            running_frame: 0,
        }
    }

    pub fn render_kid(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();

        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > 200.0 {
            self.frame_time = 0.0;
            self.running_frame += 1;
        }

        ctx.add_sprite(
            Rect::with_size(KID_X_POSITION - 1, self.y - 1, 2, 2),
            0,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            self.running_frame as usize % 4,
        );

        ctx.set_active_console(1);
        ctx.cls();
    }

    pub fn free_fall_down(&mut self) {
        if self.state == KidState::Jumping {
            self.jump_time = self.jump_time + 0.5;
            self.y -= 1;

            if self.jump_time > 1.5 {
                self.state = KidState::Falling;
                self.fall_time = DEFAULT_FALL_TIME;
                self.jump_time = 0.0;
                self.start_falling_y = self.y;
            }
        } else {
            let moved_pos = (10.0 * self.fall_time * self.fall_time / 2.0) as i32;
            self.y = self.start_falling_y + moved_pos;

            self.fall_time = self.fall_time + 0.2;
        }

        if self.y > SCREEN_HEIGHT {
            self.y = SCREEN_HEIGHT;
        }
    }

    pub fn flap(&mut self) {
        self.fall_time = DEFAULT_FALL_TIME;
        self.jump_time = 0.0;
        self.state = KidState::Jumping;
    }
}
