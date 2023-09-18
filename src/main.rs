
mod kid;
mod obstacle;
mod constants;

use bracket_lib::prelude::*;
use constants::{TOTAL_OBSTACLE_COUNT, SCREEN_HEIGHT, SCREEN_WIDTH, FRAME_DURARION, KID_X_POSITION};
use obstacle::Obstacle;
use kid::Kid;

#[derive(PartialEq)]
enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    kid: Kid,
    frame_time: f32,
    mode: GameMode,
    obstacle: [Obstacle; TOTAL_OBSTACLE_COUNT],
    score: i32
}

impl State {
    fn new() -> Self {
        State {
            kid: Kid::new(SCREEN_HEIGHT / 2),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacle: Self::create_obstacle_arr(),
            score: 0,
        }
    }

    fn create_obstacle_arr() -> [Obstacle; TOTAL_OBSTACLE_COUNT] {
        let mut obstacle_arr = [Obstacle::new(SCREEN_WIDTH); TOTAL_OBSTACLE_COUNT];
        let obs_gap = SCREEN_WIDTH / (TOTAL_OBSTACLE_COUNT) as i32;
        for i in 0..TOTAL_OBSTACLE_COUNT {
            obstacle_arr[i].x = SCREEN_WIDTH + ((i as i32) * obs_gap);
            obstacle_arr[i].gap_y = Obstacle::get_gap_y_random();
        }
        obstacle_arr
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();

        ctx.set_active_console(1);

        ctx.cls_bg(NAVY_BLUE);
        ctx.set_active_console(1);

        ctx.print_centered(10, "Game Over");
        ctx.print_centered(14, &format!("Your Score: {}", self.score));

        ctx.print_centered(20, "Press (P): Play Again");
        ctx.print_centered(22, "Press (Q): Quit Game");
        ctx.print_centered(26, "Press Space To Flap");

        self.listening_key_pressed(ctx);
    }

    fn listening_key_pressed(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quit(),
                _ => {}
            }
        }
    }

    fn render_main_menu(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);

        ctx.cls_bg(NAVY_BLUE);


        ctx.print_centered(10, "Welcome To Flappy Kid");
        ctx.print_centered(16, "Press (P): Play Game");
        ctx.print_centered(19, "Press (Q): Quit Game");
        ctx.print_centered(24, "Press Space To Flap");

        self.listening_key_pressed(ctx);
    }

    fn play_game(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY_BLUE);

        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURARION {
            self.frame_time = 0.0;
            self.kid.free_fall_down();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.kid.flap();
        }

        self.kid.render_kid(ctx);
        ctx.print_centered(2, &format!("Score: {}", self.score));

        for i in 0..TOTAL_OBSTACLE_COUNT {
            self.obstacle[i].render_view(ctx);

            if 0 > self.obstacle[i].x {
                self.obstacle[i].x = SCREEN_WIDTH;
                self.obstacle[i].gap_y = Obstacle::get_gap_y_random();
                self.obstacle[i].is_passed = false;
            }

            if KID_X_POSITION > self.obstacle[i].x && !self.obstacle[i].is_passed {
                self.score += 1;
                self.obstacle[i].is_passed = true;
            }
            if self.kid.y > SCREEN_HEIGHT || self.obstacle[i].hit_obstacle(&self.kid) {
                self.mode = GameMode::End;
                break;
            }
        }
    }

    fn restart(&mut self) {
        self.kid = Kid::new(SCREEN_HEIGHT / 2);
        self.frame_time = 0.0;

        self.mode = GameMode::Playing;
        self.score = 0;

        self.obstacle = Self::create_obstacle_arr();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Playing => self.play_game(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Menu => self.render_main_menu(ctx),
        }
    }
}

pub const TERM_UI_FONT: &str = "Bisasam_20x20.png";
embedded_resource!(UI_FONT, "../resources/Bisasam_20x20.png");
embedded_resource!(NYAN_CAT, "../resources/flappy_kid.png");

fn main() -> BError {
    link_resource!(UI_FONT, format!("resources/{}", TERM_UI_FONT));
    link_resource!(NYAN_CAT, "resources/flappy_kid.png");


    let tile_dimensions: u32 = 10;
    let tile_dimensions_height: u32 = 10;

    let ctx = BTermBuilder::new()
        .with_sprite_console(50, 40, 0)
        .with_dimensions(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .with_tile_dimensions(tile_dimensions, tile_dimensions_height)
        .with_title("Flappy Kid")
        .with_font(TERM_UI_FONT, 20u32, 20u32)
        .with_simple_console_no_bg(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, TERM_UI_FONT)
        .with_sprite_sheet(
            SpriteSheet::new("resources/flappy_kid.png")
                .add_sprite(Rect::with_size(0, 0, 85, 132))
                .add_sprite(Rect::with_size(85, 0, 85, 132))
                .add_sprite(Rect::with_size(170, 0, 85, 132))
                .add_sprite(Rect::with_size(255, 0, 85, 132)),
        )
        // .with_fps_cap(100.0)
        .with_vsync(false)
        .build()?;

    main_loop(ctx, State::new())
}
