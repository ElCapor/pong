use macroquad::prelude::*;
struct VelocityComponent {
    vx: i32,
    vy: i32,
}

impl VelocityComponent {
    /*
     *
     * Move by a specific velocity
     */
    fn set_velocity(&mut self, _vx: i32, _vy: i32) -> () {
        self.vx = _vx;
        self.vy = _vy;
    }

    fn reset_velocity(&mut self) {
        self.vx = 0;
        self.vy = 0;
    }

    fn vx(&self) -> i32 {
        self.vx
    }

    fn vy(&self) -> i32 {
        self.vy
    }
}

struct Paddle {
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
            velocity: VelocityComponent { vx: 0, vy: 0 },
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

struct Ball {
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
            velocity: VelocityComponent { vx: 0, vy: 0 },
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

    fn circle(&self) -> Circle {
        Circle {
            x: self.x as f32,
            y: self.y as f32,
            r: self.r as f32,
        }
    }

    fn rect(&self) -> Rect {
        Rect {
            x: (self.x - self.r) as f32,
            y: (self.y - self.r) as f32,
            w: self.r as f32,
            h: self.r as f32,
        }
    }

    fn reset(&mut self) {
        self.x = (screen_width() / 2.0) as i32;
        self.y = (screen_height() / 2.0) as i32;
        self.velocity.reset_velocity();
        self.velocity.set_velocity(5, 2);
    }
}

trait EventEnum {
    fn as_i32(&self) -> i32;
    fn as_str(&self) -> &'static str; // we enforce this to make debugging easier
}

enum TimerEvent {
    TimerStart,
    TimerStep,
    TimerPause,
    TimerResume,
    TimerEnd,
}

impl EventEnum for TimerEvent {
    fn as_i32(&self) -> i32 {
        return match self {
            TimerEvent::TimerStart => 0,
            TimerEvent::TimerStep => 1,
            TimerEvent::TimerPause => 2,
            TimerEvent::TimerResume => 3,
            TimerEvent::TimerEnd => 4,
        };
    }

    fn as_str(&self) -> &'static str {
        match self {
            TimerEvent::TimerStart => "TimerStart",
            TimerEvent::TimerStep => "TimerStep",
            TimerEvent::TimerPause => "TimerPause",
            TimerEvent::TimerResume => "TimerResume",
            TimerEvent::TimerEnd => "TimerEnd",
        }
    }
}

impl PartialEq for TimerEvent {
    fn eq(&self, other: &Self) -> bool {
        return self.as_i32() == other.as_i32();
    }

    fn ne(&self, other: &Self) -> bool {
        return self.as_i32() != other.as_i32();
    }
}

// Eq is what we call a marker trait: it has no method on its own, it is just a way for the programmer to express that the struct verifies a certain property.
impl Eq for TimerEvent {}

impl Hash for TimerEvent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.as_i32());
        state.finish();
    }
}

use std::{collections::HashMap, hash::Hash};

/**
 *
 * Any strcture containing this type can send events to recievers
 *
 *
 */
struct Events<T: EventEnum + Eq + Hash> {
    listeners: HashMap<T, Vec<Box<dyn Fn()>>>,
}

impl<T: EventEnum + Eq + Hash> Events<T>
where
    T: EventEnum,
{
    pub fn new() -> Events<T> {
        Events {
            listeners: HashMap::new(),
        }
    }

    pub fn listen_legacy(&mut self, etype: T, f: fn()) {
        self.listeners
            .entry(etype)
            .or_insert_with(Vec::new)
            .push(Box::new(f));
    }

    pub fn listen(&mut self, etype: T, f: Box<dyn Fn()>) {
        self.listeners.entry(etype).or_insert_with(Vec::new).push(f);
    }

    pub fn trigger(&mut self, etype: T) {
        for listener in self.listeners.entry(etype).or_insert_with(Vec::new) {
            listener();
        }
    }
}

struct Timer {
    time: i32,     // eleapsedTime since start
    end: i32,      // end time
    paused: bool,  // is_paused
    restart: bool, // should restart ?
    events: Events<TimerEvent>,
}

impl Timer {
    pub fn new(end: i32, restart: bool, autostart: bool) -> Timer {
        Timer {
            time: 0,
            end,
            paused: !autostart,
            restart,
            events: Events::new(),
        }
    }

    pub fn get_events(&mut self) -> &mut Events<TimerEvent> {
        &mut self.events
    }
    pub fn step(&mut self) {
        if self.paused {
            return;
        }

        self.time += 1;
        if self.time == 1 {
            self.events.trigger(TimerEvent::TimerStart);
        }
        self.events.trigger(TimerEvent::TimerStep);
        if self.time >= self.end {
            self.reset();
        }
    }

    pub fn pause(&mut self) {
        self.paused = true;
        self.events.trigger(TimerEvent::TimerPause);
    }

    pub fn resume(&mut self) {
        self.paused = false;
        self.events.trigger(TimerEvent::TimerResume);
    }

    pub fn reset(&mut self) {
        self.time = 0;
        if !self.restart {
            self.paused = true; // hard hack to force stop
        }
        self.events.trigger(TimerEvent::TimerEnd);
    }

    pub fn restart(&mut self) {
        self.paused = false;
    }

    pub fn time_left(&self) -> i32 {
        self.end - self.time
    }
}

#[macroquad::main("Pong")]
async fn main() {
    let mut player1: Paddle = Paddle::new(0, screen_width() as i32 / 2, 20, 80, WHITE);
    let mut player2: Paddle = Paddle::new(
        screen_width() as i32 - 20,
        screen_height() as i32 / 2,
        20,
        80,
        WHITE,
    );
    let mut ball: Ball = Ball::new(
        (screen_width() / 2.0) as i32,
        (screen_height() / 2.0) as i32,
        15,
        WHITE,
    );

    ball.velocity.set_velocity(6, -2);
    loop {
        let minimum_frame_time = 1. / 60.; // 60 FPS
        let frame_time = get_frame_time();
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }
        if is_key_down(KeyCode::Down) {
            player1.start_move_down();
        } else if is_key_down(KeyCode::Up) {
            player1.start_move_up();
        } else {
            player1.stop_move();
        }

        if is_key_down(KeyCode::Z) {
            player2.start_move_up();
        } else if is_key_down(KeyCode::S) {
            player2.start_move_down();
        } else {
            player2.stop_move();
        }
        if player1.rect().intersect(ball.rect()).is_some()
            || player2.rect().intersect(ball.rect()).is_some()
        {
            if ball.dirty_timer.time == 0 {
                let offset = ball.rect().y - player1.rect().y;
                ball.velocity
                    .set_velocity(-ball.velocity.vx(), ball.velocity.vy());
                println!("{}", offset);
                ball.dirty_timer.restart();
            }
        }
        player2.update();
        player1.update();
        ball.update();
        // The background
        clear_background(BLACK);

        // draw the first pad
        player1.draw();
        player2.draw();
        ball.draw();
        next_frame().await
    }
}
