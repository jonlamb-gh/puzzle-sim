use crate::na::{self, Isometry3, Point3, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window;
use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::joint::FreeJoint;
use nphysics3d::object::{
    BodyHandle, BodyPartHandle, ColliderDesc, ColliderHandle, MultibodyDesc, RigidBodyDesc,
};
use nphysics3d::world::World;
use nphysics_testbed3d::objects::box_node::Box;

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

pub struct L100Bar {
    body: BodyHandle,
    node: Box,
}

impl L100Bar {
    pub fn new(
        world: &mut World<f32>,
        translation: Vector3<f32>,
        color: Point3<f32>,
        window: &mut window::Window,
    ) -> Self {
        let delta = na::one();
        //let size = Vector3::new(100.0, 20.0, 20.0);
        let size = Vector3::new(100.0, 5.0, 5.0);
        let shape = Cuboid::new(size / 2.0);

        let collider_desc = ColliderDesc::new(ShapeHandle::new(shape.clone()))
            // Mass and angular inertia will be added to this rigid body
            .density(1.0)
            .translation(translation);

        //let free = FreeJoint::new(Isometry3::identity());
        //        let free = FreeJoint::new(Isometry3::new(translation, na::zero()));
        //        let mbody = MultibodyDesc::new(free)
        //            .collider(&collider_desc)
        //            .build_with_parent(BodyPartHandle::ground(), world)
        //            .unwrap();

        //let collider = collider_desc.build(world);

        //let margin = collider.margin();
        let margin = collider_desc.get_margin();
        let rx = shape.half_extents().x + margin;
        let ry = shape.half_extents().y + margin;
        let rz = shape.half_extents().z + margin;

        let rb = RigidBodyDesc::new()
            .mass(1.0)
            .collider(&collider_desc)
            .build(world);

        let rb_handle = rb.handle();

        let collider = world
            .collider_world()
            .body_colliders(rb_handle)
            .next()
            .unwrap();

        L100Bar {
            body: collider.body(),
            node: Box::new(collider.handle(), world, delta, rx, ry, rz, color, window),
        }
    }
}

impl Bar for L100Bar {
    fn update(&mut self, world: &World<f32>) {
        self.node.update(world);
    }

    fn scene_node(&self) -> &SceneNode {
        self.node.scene_node()
    }

    fn object(&self) -> ColliderHandle {
        self.node.object()
    }

    fn body(&self) -> BodyHandle {
        self.body
    }
}
