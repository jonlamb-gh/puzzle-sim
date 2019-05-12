// units: centimeters

// TODO
//   - subtract instead of add margin on shapes

mod bar;
mod builder;
mod ground;
mod puzzle;

use nalgebra as na;

use crate::builder::Builder;
use crate::ground::Ground;
use crate::na::{Point3, Vector3};
use crate::puzzle::Puzzle;
use kiss3d::camera::{ArcBall, Camera};
use kiss3d::light::Light;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::window::{State, Window};
use nphysics3d::world::World;

struct AppState {
    arc_ball: ArcBall,
    world: World<f32>,
    ground: Ground,
    puzzle: Puzzle,
}

impl AppState {
    pub fn new(window: &mut Window) -> Self {
        let arc_ball = ArcBall::new(Point3::new(-20.0, 10.0, -20.0), Point3::new(0.0, 0.0, 0.0));

        let mut world = World::new();
        //world.set_contact_model(SignoriniModel::new());
        world.set_gravity(Vector3::new(0.0, -9.81, 0.0));

        let ground = Ground::new(&mut world, window);

        let puzzle = Builder::build(&mut world, window);

        AppState {
            arc_ball,
            world,
            ground,
            puzzle,
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

            self.puzzle.update(&self.world);
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
