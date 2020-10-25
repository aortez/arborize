extern crate amethyst;

mod pong
{
    use amethyst::ecs::prelude::*;

    pub enum Side
    {
        Left,
        Right,
    }
    pub struct Paddle
    {
        pub side: Side,
    }
    impl Component for Paddle
    {
        type Storage = VecStorage<Self>;
    }

    pub const ARENA_HEIGHT: f32 = 200.0;
    pub const PADDLE_HEIGHT: f32 = 16.0;
}

use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::window::ScreenDimensions;

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem
{
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            paddles,
            screen_dimensions,
            input,
        ): Self::SystemData,
    ) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;
                let paddle_y = transform.translation().y;
                transform.set_translation_y(
                    (paddle_y + scaled_amount)
                        .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                        .max(0.0 + PADDLE_HEIGHT * 0.5),
                );
                println!("paddle at: {}, {}", transform.translation().x, transform.translation().y);
            }

            if let Some((x, y)) = input.mouse_position() {
                println!("mouse at: {}, {}", x, y);
                println!("screen dims: {}, {}", screen_dimensions.width(), screen_dimensions.height());
            }
        }
    }
}
