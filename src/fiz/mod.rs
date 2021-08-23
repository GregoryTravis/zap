extern crate nalgebra as na;
extern crate nphysics3d;

use na::Vector3;
use nphysics3d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics3d::object::{RigidBodyDesc};
// use nphysics3d::math::{Velocity, Inertia};
use ncollide3d::shape::{ShapeHandle, Ball};
// use ncollide3d::world::CollisionGroups;
use nphysics3d::object::ColliderDesc;
// use nphysics3d::material::{MaterialHandle, BasicMaterial};
use nphysics3d::object::BodyPartHandle;

pub fn fiz_main() {
    let mut mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));
    let mut geometrical_world = DefaultGeometricalWorld::new();

    let rigid_body = RigidBodyDesc::new()
        .translation(Vector3::y() * 5.0)
        .mass(1.2)
        .build();

    let mut bodies = DefaultBodySet::new();
    let handle = bodies.insert(rigid_body);
    // let body = bodies.get(handle);

    let shape = ShapeHandle::new(Ball::new(1.5));
    let collider = ColliderDesc::new(shape)
        .translation(Vector3::y() * 5.0)
        .build(BodyPartHandle(handle, 0));

    let mut colliders = DefaultColliderSet::new();
    let _collider_handle = colliders.insert(collider);
    let mut joint_constraints = DefaultJointConstraintSet::new();
    let mut force_generators = DefaultForceGeneratorSet::new();

    loop {
        // println!("ooo");
        // Run the simulation.
        mechanical_world.step(
            &mut geometrical_world,
            &mut bodies,
            &mut colliders,
            &mut joint_constraints,
            &mut force_generators
        );
        return;
    }
}
