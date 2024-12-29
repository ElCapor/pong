use super::engine::{velocity::VelocityComponent};
use macroquad::prelude::*;

pub struct Paddle {
    x: i32, // we keep x cuz we can reuse it to do cool fun animations
    y: i32,
    w: i32,
    h: i32,
    color: macroquad::color::Color,
    velocity: VelocityComponent,
}

impl Paddle {
    pub fn new(x: i32, y: i32, w: i32, h: i32, color: macroquad::color::Color) -> Paddle {
        Paddle {
            x,
            y,
            w,
            h,
            color,
            velocity: VelocityComponent::new(0, 0),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.x as f32,
            self.y as f32,
            self.w as f32,
            self.h as f32,
            self.color,
        );
    }

    pub fn update(&mut self) {
        self.x += self.velocity.vx();
        self.y += self.velocity.vy();

        self.y = if self.y < 0 {
            0
        } else if (self.y + self.h) > screen_height() as i32 {
            screen_height() as i32 - self.h
        } else {
            self.y
        }
    }
    /*
     * We don't make a generic update with key function because we want to make this compatible
     * with networking later on...
     */
    pub fn start_move_down(&mut self) {
        self.velocity.set_velocity(0, 5);
    }

    pub fn start_move_up(&mut self) {
        self.velocity.set_velocity(0, -5);
    }

    pub fn stop_move(&mut self) {
        self.velocity.reset_velocity();
    }

    pub fn rect(&self) -> Rect {
        Rect {
            x: self.x as f32,
            y: self.y as f32,
            w: self.w as f32,
            h: self.h as f32,
        }
    }
}