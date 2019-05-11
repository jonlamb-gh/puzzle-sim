use crate::bar::Bar;
use nphysics3d::object::BodyHandle;
use nphysics3d::world::World;
use std::collections::HashMap;

pub struct Puzzle {
    bars: HashMap<BodyHandle, Box<Bar>>,
}

impl Puzzle {
    pub fn new(bars: HashMap<BodyHandle, Box<Bar>>) -> Self {
        Puzzle { bars }
    }

    pub fn update(&mut self, world: &World<f32>) {
        for bar in self.bars.values_mut() {
            bar.update(world);
        }
    }
}
