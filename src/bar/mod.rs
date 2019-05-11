// TODO - some trait?
//
// a bar has:
//   - zero or more PinSlotJoints/etc, knowns their relative positions
//   - size/shape
//   - caller maintains position
//   - get_??() - get child handles (pin joints/etc)
//
// classes/types of bars:
//   - StandardBar - solid bar of any size/shape? or unique to size/shape?
//   - L100Bar - 100 mm bar
//   - L50Bar
//   - XyzBar
//   - ZzzBar

mod l100;

pub use l100::L100 as L100Bar;

use kiss3d::scene::SceneNode;
use nphysics3d::object::{BodyHandle, ColliderHandle};
use nphysics3d::world::World;

// parent Part trait? Pin and Bar share a lot
pub trait Bar {
    fn update(&mut self, world: &World<f32>);

    fn scene_node(&self) -> &SceneNode;

    fn object(&self) -> ColliderHandle;

    fn body(&self) -> BodyHandle;
}

// move to mod file
// struct with Vec/map of pins/joints/etc
// build size/shape from config.rs constants
