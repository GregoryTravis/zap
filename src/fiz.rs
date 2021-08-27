extern crate nalgebra as na;
extern crate nphysics2d;

use na::Vector;
use na::Vector2;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics2d::object::{RigidBodyDesc};
use nphysics2d::math::{Velocity};
use ncollide2d::shape::{ShapeHandle, Ball};
use nphysics2d::object::ColliderDesc;
use nphysics2d::object::BodyPartHandle;

pub fn fiz_main() {
    let mut mechanical_world = DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0));
    let mut geometrical_world = DefaultGeometricalWorld::new();

    let rigid_body = RigidBodyDesc::new()
        .translation(Vector::y() * 5.0)
        .mass(1.2)
        // .gravity_enabled(true)
        // .set_status(BodyStatus::Kinematic)
        .velocity(Velocity::linear(1.0, 2.0))
        .build();

    let mut bodies = DefaultBodySet::new();
    let handle = bodies.insert(rigid_body);
    // let body = bodies.get(handle);

    let shape = ShapeHandle::new(Ball::new(1.5));
    let collider = ColliderDesc::new(shape)
        .translation(Vector::y() * 5.0)
        .build(BodyPartHandle(handle, 0));

    let mut colliders = DefaultColliderSet::new();
    let _collider_handle = colliders.insert(collider);
    let mut joint_constraints = DefaultJointConstraintSet::new();
    let mut force_generators = DefaultForceGeneratorSet::new();

    // loop {
    for i in 0..4 {
        // println!("ooo");
        // Run the simulation.
        mechanical_world.step(
            &mut geometrical_world,
            &mut bodies,
            &mut colliders,
            &mut joint_constraints,
            &mut force_generators
        );
        let body = bodies.rigid_body(handle).unwrap();
        let _x = body.position().translation.x;
        println!("oy {} {}", i, body.position());
        println!("oy2 {}", mechanical_world.timestep());
// let platform = bodies.rigid_body_mut(platform_handle).unwrap();
//         let platform_z = platform.position().translation.z;
    }
}
