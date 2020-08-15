extern crate amethyst;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

mod pong {
    use amethyst::ecs::prelude::*;

    pub struct Ball {
       pub velocity: [f32; 2],
       pub radius: f32,
    }
    impl Component for Ball {
       type Storage = DenseVecStorage<Self>;
    }

    #[derive(PartialEq, Eq)]
    pub enum Side {
      Left,
      Right,
    }

    pub struct Paddle {
      pub side: Side,
      pub width: f32,
      pub height: f32,
    }
    impl Component for Paddle {
      type Storage = VecStorage<Self>;
    }

    pub const ARENA_HEIGHT: f32 = 100.0;
    pub const ARENA_WIDTH: f32 = 100.0;
}

use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::pong::{Ball, Side, Paddle, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, paddles, transforms): Self::SystemData) {
        // Check whether a ball collided, and bounce off accordingly.
        //
        // We also check for the velocity of the ball every time, to prevent multiple collisions
        // from occurring.
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            // Bounce at the top or the bottom of the arena.
            if (ball_y <= ball.radius && ball.velocity[1] < 0.0)
                || (ball_y >= ARENA_HEIGHT - ball.radius && ball.velocity[1] > 0.0)
            {
                ball.velocity[1] = -ball.velocity[1];
            }

            // Bounce off sides.
            if (ball_x <= ball.radius && ball.velocity[0] < 0.0)
                || (ball_x >= ARENA_WIDTH - ball.radius && ball.velocity[0] > 0.0)
            {
                ball.velocity[0] = -ball.velocity[0];
            }

            // Bounce at the paddles.
            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x - (paddle.width * 0.5);
                let paddle_y = paddle_transform.translation().y - (paddle.height * 0.5);

                // Bounce off paddles.
                if point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - ball.radius,
                    paddle_y - ball.radius,
                    paddle_x + paddle.width + ball.radius,
                    paddle_y + paddle.height + ball.radius,
                ) {
                    if (paddle.side == Side::Left && ball.velocity[0] < 0.0)
                        || (paddle.side == Side::Right && ball.velocity[0] > 0.0)
                    {
                        ball.velocity[0] = -ball.velocity[0];
                    }
                }
            }
        }
    }
}

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
