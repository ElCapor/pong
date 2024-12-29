use macroquad::prelude::*;
mod game;

use game::{paddle::Paddle, ball::Ball};

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

    ball.velocity_mut().set_velocity(6, -2);
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
            if ball.dirty_timer_mut().time() == 0 {
                let offset = ball.rect().y - player1.rect().y;
                let vx = -ball.velocity().vx();
                let vy = ball.velocity().vy();
                ball.velocity_mut()
                    .set_velocity(vx,vy);
                println!("{}", offset);
                ball.dirty_timer_mut().restart();
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
