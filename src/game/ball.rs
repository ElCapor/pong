
use super::engine::{events::Events, velocity::VelocityComponent, timer::Timer};
use macroquad::prelude::*;
pub struct Ball {
    x: i32,
    y: i32,
    r: i32, //radius
    color: macroquad::color::Color,
    velocity: VelocityComponent,
    dirty_timer: Timer,
}

impl Ball {
    pub fn new(x: i32, y: i32, r: i32, color: macroquad::color::Color) -> Ball {
        Ball {
            x,
            y,
            r,
            color,
            velocity: VelocityComponent::new(0, 0),
            dirty_timer: Timer::new(30, false, false),
        }
    }

    pub fn update(&mut self) {
        self.dirty_timer.step();
        self.x += self.velocity.vx();
        self.y += self.velocity.vy();

        if self.x < self.r || self.x + self.r > screen_width() as i32 + self.r as i32 {
            /*
            self.velocity
                .set_velocity(-self.velocity.vx(), self.velocity.vy());
            */
            self.reset();
        }
        if self.y < self.r || self.y + self.r > screen_height() as i32 {
            self.velocity
                .set_velocity(self.velocity.vx(), -self.velocity.vy());
        }
    }

    pub fn draw(&self) {
        draw_circle(self.x as f32, self.y as f32, self.r as f32, self.color);
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn circle(&self) -> Circle {
        Circle {
            x: self.x as f32,
            y: self.y as f32,
            r: self.r as f32,
        }
    }

    pub fn rect(&self) -> Rect {
        Rect {
            x: (self.x - self.r) as f32,
            y: (self.y - self.r) as f32,
            w: self.r as f32,
            h: self.r as f32,
        }
    }

    pub fn reset(&mut self) {
        self.x = (screen_width() / 2.0) as i32;
        self.y = (screen_height() / 2.0) as i32;
        self.velocity.reset_velocity();
        self.velocity.set_velocity(5, 2);
    }

    pub fn velocity_mut(&mut self) -> &mut VelocityComponent
    {
        &mut self.velocity
    }

    pub fn velocity(&self) -> &VelocityComponent{
        &self.velocity
    }
    pub fn dirty_timer_mut(&mut self) -> &mut Timer
    {
        &mut self.dirty_timer
    }
}