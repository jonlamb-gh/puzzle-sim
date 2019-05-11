use crate::bar::{Bar, L100Bar};
use crate::na::{Point3, Vector3};
use crate::puzzle::Puzzle;
use kiss3d::window;
use nphysics3d::object::BodyHandle;
use nphysics3d::world::World;
use std::collections::HashMap;

pub struct Builder {}

impl Builder {
    pub fn build(world: &mut World<f32>, window: &mut window::Window) -> Puzzle {
        let mut bars = HashMap::<BodyHandle, Box<Bar>>::new();

        let l100_bar = L100Bar::new(
            world,
            Vector3::new(0.0, 3.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            window,
        );
        let _ = bars.insert(l100_bar.body(), Box::new(l100_bar));

        let l100_bar = L100Bar::new(
            world,
            Vector3::new(0.0, 6.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            window,
        );
        let _ = bars.insert(l100_bar.body(), Box::new(l100_bar));

        Puzzle::new(bars)
    }
}
