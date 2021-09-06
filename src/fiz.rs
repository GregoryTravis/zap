extern crate nalgebra as na;
extern crate nphysics2d;

// use na::Vector;
use na::Vector2;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet, Ground};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{// MechanicalWorld, GeometricalWorld,
  DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics2d::object::{RigidBodyDesc};
use nphysics2d::math::{Velocity};
use ncollide2d::shape::{ShapeHandle, Ball, Cuboid};
use nphysics2d::object::ColliderDesc;
use nphysics2d::object::{BodyPartHandle, DefaultColliderHandle, DefaultBodyHandle};

use crate::thing::Thing;

pub struct FizThing {
  // #[allow(dead_code)] // TODO remove
  // fiz: &'a mut Fiz,
  #[allow(dead_code)] // TODO remove
  pub body_handle: DefaultBodyHandle,
  #[allow(dead_code)] // TODO remove
  collider_handle: DefaultColliderHandle, 
}

pub struct Fiz {
  pub mechanical_world: DefaultMechanicalWorld<f32>,
  geometrical_world: DefaultGeometricalWorld<f32>,
  joint_constraints: DefaultJointConstraintSet<f32>,
  force_generators: DefaultForceGeneratorSet<f32>,
  pub body_set: DefaultBodySet<f32>,
  collider_set: DefaultColliderSet<f32>,
}

impl Fiz {
  pub fn new() -> Fiz {
    return Fiz {
      mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0)),
      geometrical_world: DefaultGeometricalWorld::new(),
      joint_constraints: DefaultJointConstraintSet::new(),
      force_generators: DefaultForceGeneratorSet::new(),
      body_set: DefaultBodySet::new(),
      collider_set:  DefaultColliderSet::new(),
    };
  }

  pub fn add_thing(&mut self, _thing: Thing, position: Vector2<f32>, velocity: Vector2<f32>) -> FizThing {
    let rigid_body = RigidBodyDesc::new()
        .translation(position)
        .mass(1.2)
        // .gravity_enabled(true)
        // .set_status(BodyStatus::Kinematic)
        .velocity(Velocity::linear(velocity.x, velocity.y))
        .build();

    let body_handle = self.body_set.insert(rigid_body);

    let shape = ShapeHandle::new(Ball::new(1.5));
    let collider = ColliderDesc::new(shape)
        .translation(position)
        .density(1.0)
        .material(MaterialHandle::new(BasicMaterial::new(0.3, 0.8)))
        .build(BodyPartHandle(body_handle, 0));

    let collider_handle = self.collider_set.insert(collider);

    return FizThing {
      // fiz: self,
      body_handle: body_handle,
      collider_handle: collider_handle,
    }
  }

  pub fn add_collider(&mut self, dim: Vector2<f32>, pos: Vector2<f32>) {
    // wall
    // let ground_size = r!(5.0);
    let ground_shape = ShapeHandle::new(Cuboid::new(dim));

    let ground_handle = self.body_set.insert(Ground::new());
    let co = ColliderDesc::new(ground_shape)
        .translation(pos)
        .density(1.0)
        .material(MaterialHandle::new(BasicMaterial::new(0.3, 0.8)))
        .build(BodyPartHandle(ground_handle, 0));
    self.collider_set.insert(co);
  }

  pub fn step(&mut self) {
    self.mechanical_world.step(
        &mut self.geometrical_world,
        &mut self.body_set,
        &mut self.collider_set,
        &mut self.joint_constraints,
        &mut self.force_generators
    );
  }
}
