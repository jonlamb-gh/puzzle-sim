mod bar;

use nalgebra as na;

use crate::bar::{Bar, L100Bar};
use crate::na::{Point3, Vector3};
use kiss3d::camera::{ArcBall, Camera};
use kiss3d::light::Light;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::window::{State, Window};
use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::object::{BodyHandle, ColliderDesc};
use nphysics3d::world::World;
use nphysics_testbed3d::objects::{box_node, node::Node};
use std::collections::HashMap;

//struct AppState<T> {
struct AppState {
    arc_ball: ArcBall,
    world: World<f32>,
    ground: Node,
    // drawables, generic T?
    //bars: HashMap<BodyHandle, T>,
    //bars: HashMap<BodyHandle, Box<Bar + Send + Sync>>,
    bars: HashMap<BodyHandle, Box<Bar>>,
}

//impl<T> AppState<T>
//where
//    T: Bar + Send + Sync,
impl AppState {
    pub fn new(window: &mut Window) -> Self {
        let arc_ball = ArcBall::new(Point3::new(-10.0, 10.0, -10.0), Point3::new(0.0, 0.0, 0.0));

        let mut world = World::new();
        world.set_gravity(Vector3::new(0.0, -9.81, 0.0));

        let ground_size = Vector3::new(25.0, 0.5, 25.0);
        let ground_shape = ShapeHandle::new(Cuboid::new(ground_size));

        let ground_ch = ColliderDesc::new(ground_shape)
            .translation(Vector3::y() * -0.5)
            .build(&mut world);

        let object = ground_ch.handle();

        // TODO
        let collider = world.collider(object).unwrap();
        let shape = collider.shape().as_ref();
        let shape = shape.as_shape::<Cuboid<f32>>().unwrap();

        let margin = world.collider(object).unwrap().margin();
        let rx = shape.half_extents().x + margin;
        let ry = shape.half_extents().y + margin;
        let rz = shape.half_extents().z + margin;
        let delta = na::one();

        let color = Point3::new(0.4, 0.4, 0.4);

        let ground = Node::Box(box_node::Box::new(
            object, &mut world, delta, rx, ry, rz, color, window,
        ));

        let l100_bar = L100Bar::new(
            &mut world,
            Vector3::new(0.0, 5.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            window,
        );

        let mut bars = HashMap::<BodyHandle, Box<Bar>>::new();

        let _ = bars.insert(l100_bar.body(), Box::new(l100_bar));

        AppState {
            arc_ball,
            world,
            ground,
            bars,
        }
    }
}

//impl<T: 'static> State for AppState<T>
//where
//    T: Bar + Send + Sync,
impl State for AppState {
    fn cameras_and_effect(
        &mut self,
    ) -> (
        Option<&mut Camera>,
        Option<&mut PlanarCamera>,
        Option<&mut PostProcessingEffect>,
    ) {
        (Some(&mut self.arc_ball), None, None)
    }

    fn step(&mut self, win: &mut Window) {
        if !win.is_closed() && !win.should_close() {
            self.world.step();

            self.ground.update(&self.world);

            for bar in self.bars.values_mut() {
                bar.update(&self.world);
            }
        }
    }
}

fn main() {
    let mut window = Window::new("Window");
    window.set_framerate_limit(Some(60));
    window.set_light(Light::StickToCamera);
    window.set_background_color(0.9, 0.9, 0.9);

    let state = AppState::new(&mut window);

    window.render_loop(state)
}
