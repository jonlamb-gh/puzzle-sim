use nalgebra as na;

use crate::na::geometry::UnitQuaternion;
use crate::na::{Isometry3, Point2, Point3, Vector3};
use kiss3d::camera::{ArcBall, Camera};
use kiss3d::light::Light;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::text::Font;
use kiss3d::window::{State, Window};
use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::math::Velocity;
use nphysics3d::object::BodyHandle;
use nphysics3d::object::{ColliderDesc, RigidBodyDesc};
use nphysics3d::world::World;
use nphysics_testbed3d::objects::{box_node::Box, node::Node};

struct AppState {
    arc_ball: ArcBall,
    world: World<f32>,
    ground: Node,
}

impl AppState {
    pub fn new(window: &mut Window) -> Self {
        let arc_ball = ArcBall::new(Point3::new(-10.0, 10.0, -10.0), Point3::new(0.0, 0.0, 0.0));

        let mut world = World::new();
        world.set_gravity(Vector3::new(0.0, -9.81, 0.0));

        let ground_size = Vector3::new(50.0, 1.0, 50.0);
        let ground_shape = ShapeHandle::new(Cuboid::new(ground_size));

        let ground_ch = ColliderDesc::new(ground_shape)
            .translation(Vector3::y() * -ground_size.x)
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

        let ground = Node::Box(Box::new(
            object, &mut world, delta, rx, ry, rz, color, window,
        ));

        AppState {
            arc_ball,
            world,
            ground,
        }
    }
}

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
