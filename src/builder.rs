use crate::bar::{Bar, L100Bar};
use crate::na::{Point3, Vector3};
use crate::puzzle::Puzzle;
use kiss3d::window;
use nphysics3d::object::BodyHandle;
use nphysics3d::world::World;
use std::collections::HashMap;

// units: centimeters

const MARGIN_OFFSET: f32 = 0.01;

const UNIT_DIM: f32 = 2.0;
const NUM_UNITS: usize = 5;
const MIN_SPACING: f32 = 0.1;

const L0_BASE_Y: f32 = 0.1 + UNIT_DIM / 2.0;
const L1_BASE_Y: f32 = L0_BASE_Y + UNIT_DIM + MIN_SPACING;
const L2_BASE_Y: f32 = L1_BASE_Y + UNIT_DIM + MIN_SPACING;
const L3_BASE_Y: f32 = L2_BASE_Y + UNIT_DIM + MIN_SPACING;
const L4_BASE_Y: f32 = L3_BASE_Y + UNIT_DIM + MIN_SPACING;

const RED: [f32; 3] = [1.0, 0.0, 0.0];
const GREEN: [f32; 3] = [0.0, 1.0, 0.0];
const BLUE: [f32; 3] = [0.0, 0.0, 1.0];

pub struct Builder {}

impl Builder {
    pub fn build(world: &mut World<f32>, window: &mut window::Window) -> Puzzle {
        let mut bars = HashMap::<BodyHandle, Box<Bar>>::new();

        // Base Y (up) starts at 2*margin
        // Builds in the positive Z direction

        // TODO - use a builder pattern
        // center Z
        Self::base_layer_0(world, window, &mut bars);
        Self::base_layer_1(world, window, &mut bars);
        Self::base_layer_2(world, window, &mut bars);
        Self::base_layer_3(world, window, &mut bars);
        Self::base_layer_4(world, window, &mut bars);

        Puzzle::new(bars)
    }

    fn base_layer_0(
        world: &mut World<f32>,
        window: &mut window::Window,
        bars: &mut HashMap<BodyHandle, Box<Bar>>,
    ) {
        for u in 0..NUM_UNITS {
            let x = 0.0;
            let y = L0_BASE_Y;
            let z = (UNIT_DIM + MIN_SPACING) * (u as f32);

            let l100_bar = L100Bar::new(world, Vector3::new(x, y, z), GREEN.into(), window);
            bars.insert(l100_bar.body(), Box::new(l100_bar))
                .map(|_| panic!("HashMap already has entry"));
        }
    }

    fn base_layer_1(
        world: &mut World<f32>,
        window: &mut window::Window,
        bars: &mut HashMap<BodyHandle, Box<Bar>>,
    ) {
        for u in 0..NUM_UNITS {
            let x = 0.0;
            let y = L1_BASE_Y;
            let z = -1.2 + (UNIT_DIM + MIN_SPACING) * (u as f32);

            let l100_bar = L100Bar::new(world, Vector3::new(x, y, z), RED.into(), window);
            bars.insert(l100_bar.body(), Box::new(l100_bar))
                .map(|_| panic!("HashMap already has entry"));
        }
    }

    fn base_layer_2(
        world: &mut World<f32>,
        window: &mut window::Window,
        bars: &mut HashMap<BodyHandle, Box<Bar>>,
    ) {
        for u in 0..NUM_UNITS {
            let x = 0.0;
            let y = L2_BASE_Y;
            let z = (UNIT_DIM + MIN_SPACING) * (u as f32);

            let l100_bar = L100Bar::new(world, Vector3::new(x, y, z), BLUE.into(), window);
            bars.insert(l100_bar.body(), Box::new(l100_bar))
                .map(|_| panic!("HashMap already has entry"));
        }
    }

    fn base_layer_3(
        world: &mut World<f32>,
        window: &mut window::Window,
        bars: &mut HashMap<BodyHandle, Box<Bar>>,
    ) {
        for u in 0..NUM_UNITS {
            let x = 0.0;
            let y = L3_BASE_Y;
            let z = (UNIT_DIM + MIN_SPACING) * (u as f32);

            let l100_bar = L100Bar::new(world, Vector3::new(x, y, z), GREEN.into(), window);
            bars.insert(l100_bar.body(), Box::new(l100_bar))
                .map(|_| panic!("HashMap already has entry"));
        }
    }

    fn base_layer_4(
        world: &mut World<f32>,
        window: &mut window::Window,
        bars: &mut HashMap<BodyHandle, Box<Bar>>,
    ) {
        for u in 0..NUM_UNITS {
            let x = 0.0;
            let y = L4_BASE_Y;
            let z = (UNIT_DIM + MIN_SPACING) * (u as f32);

            let l100_bar = L100Bar::new(world, Vector3::new(x, y, z), RED.into(), window);
            bars.insert(l100_bar.body(), Box::new(l100_bar))
                .map(|_| panic!("HashMap already has entry"));
        }
    }
}
