extern crate amethyst;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

mod pong
{
    use amethyst::ecs::prelude::*;

    pub struct Ball
    {
        pub velocity: [f32; 2],
        pub radius: f32,
    }
    impl Component for Ball
    {
        type Storage = DenseVecStorage<Self>;
    }
}

use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::pong::Ball;

#[derive(SystemDesc)]
pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem
{
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, mut locals, time): Self::SystemData)
    {
        // Move every ball according to its speed, and the time passed.
        for (ball, local) in (&balls, &mut locals).join() {
            local.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            local.prepend_translation_y(ball.velocity[1] * time.delta_seconds());
        }
    }
}

fn main() {}
