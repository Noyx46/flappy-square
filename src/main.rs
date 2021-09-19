//! A work-in-progress game based off of Flappy Bird.

use ggez::conf;
use ggez::event::{self, KeyCode, KeyMods};
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};
use glam::*;
use oorandom;

const BAR_INTERVAL: f32 = 2.0;
const BAR_WIDTH: f32 = 40.0;
const BIRD_SIZE: f32 = 30.0;
const BIRD_START_Y: f32 = 40.0;
const BIRD_X: f32 = 100.0;
const GAP_SIZE: f32 = 80.0;
const GRAVITY: f32 = 0.2;
const JUMP: f32 = 4.0;
const SPEED: f32 = 4.0;

/// A rectangle to collide with the bird, 2 per obstacle
struct Bar {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[allow(dead_code)]
struct MainState {
    pos_y: f32,
    vel_y: f32,
    screen_width: f32,
    screen_height: f32,
    bars: Vec<Bar>,
    // In seconds
    bar_cooldown: f32,
    rng: oorandom::Rand32,
    dead: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // Use a specified seed for now; change later
        let rng = oorandom::Rand32::new(8);

        let (screen_width, screen_height) = graphics::drawable_size(ctx);
        Ok(MainState {
            pos_y: BIRD_START_Y,
            vel_y: 0.0,
            screen_width,
            screen_height,
            bars: vec![],
            bar_cooldown: 0.0,
            rng,
            dead: false,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const FPS: u32 = 60;
        while timer::check_update_time(ctx, FPS) && !self.dead {
            let seconds = 1.0 / (FPS as f32);

            // Check collision
            for bar in &self.bars {
                let left = BIRD_X;
                let right = BIRD_X + BIRD_SIZE;
                let top = self.pos_y;
                let bottom = self.pos_y + BIRD_SIZE;

                if left < bar.x + bar.width
                    && right > bar.x
                    && top < bar.y + bar.height
                    && bottom > bar.y
                {
                    self.dead = true;
                }
            }

            // Spawn bars
            if self.bar_cooldown <= 0.0 {
                // Get a value between 10 and screen_size - 10 - GAP_SIZE
                let space = (self.screen_height - 20.0 - GAP_SIZE) * self.rng.rand_float() + 10.0;
                // Create bottom and top bar
                let top_bar = Bar {
                    x: self.screen_width,
                    y: 0.0,
                    width: BAR_WIDTH,
                    height: space,
                };
                let bottom_bar = Bar {
                    x: self.screen_width,
                    y: space + GAP_SIZE,
                    width: BAR_WIDTH,
                    height: self.screen_height - GAP_SIZE - space,
                };
                self.bars.push(top_bar);
                self.bars.push(bottom_bar);

                // Reset cooldown
                self.bar_cooldown = BAR_INTERVAL;
            } else {
                self.bar_cooldown -= seconds;
            }

            // Move Bars
            for i in (0..self.bars.len()).rev() {
                self.bars[i].x -= SPEED;
                if self.bars[i].x + BAR_WIDTH < 0.0 {
                    let _ = self.bars.remove(i);
                }
            }

            // Bird Physics
            self.vel_y = 50f32.min(self.vel_y + GRAVITY);
            self.pos_y += self.vel_y;

            // Make sure bird is on-screen
            if self.pos_y < 0.0 {
                self.pos_y = 0.0;
                self.vel_y = 0.0;
            } else if self.pos_y > self.screen_height - BIRD_SIZE {
                self.pos_y = self.screen_height - BIRD_SIZE;
                self.vel_y = 0.0;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::new(0.1, 0.1, 0.1, 1.0));

        // Draw all bars
        for bar in &self.bars {
            let drawn_bar = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, bar.width, bar.height),
                graphics::Color::new(0.1, 0.8, 0.1, 1.0),
            )?;
            graphics::draw(ctx, &drawn_bar, ([bar.x, bar.y],))?;
        }

        // Time to attempt to create a rectangle
        let bird = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, BIRD_SIZE, BIRD_SIZE),
            graphics::Color::new(0.8, 0.1, 0.1, 1.0),
        )?;
        graphics::draw(ctx, &bird, ([BIRD_X, self.pos_y],))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        repeat: bool,
    ) {
        if repeat == false {
            match keycode {
                KeyCode::Space => {
                    self.vel_y = -JUMP;
                }
                KeyCode::R => {
                    self.pos_y = BIRD_START_Y;
                    self.vel_y = 0.0;
                    self.bars = vec![];
                    self.bar_cooldown = 0.0;
                    self.dead = false;
                }
                _ => (),
            }
        }
    }
}

fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("flappy square", "ggez")
        .window_setup(conf::WindowSetup::default().title("Flappy"))
        .window_mode(conf::WindowMode::default().dimensions(1000.0, 500.0));
    let (mut ctx, event_loop) = context_builder.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
