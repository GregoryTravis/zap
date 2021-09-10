extern crate nalgebra as na;
extern crate nphysics2d;

// use na::Vector;
use na::Vector2;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet, BodyStatus};
use na::{Vector3, Isometry3};
use crate::thing::transform;
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{// MechanicalWorld, GeometricalWorld,
  DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics2d::object::{RigidBodyDesc};
use nphysics2d::math::{Velocity};
// use ncollide2d::shape::{ShapeHandle, ConvexPolygon, Cuboid};
use nphysics2d::object::ColliderDesc;
use nphysics2d::object::{BodyPartHandle, DefaultColliderHandle, DefaultBodyHandle};
// use ncollide2d::narrow_phase::ContactEvent::Started;

use crate::thing::Thing;

pub struct FizThing {
  // #[allow(dead_code)] // TODO remove
  // fiz: &'a mut Fiz,
  pub thing: Thing,
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

  pub fn add_thing(&mut self, thing: &Thing, position: Vector2<f32>, rotation: f32, velocity: Vector2<f32>, is_wall: bool) -> FizThing {

    let body_status = if is_wall { BodyStatus::Static } else { BodyStatus::Dynamic };
    let rigid_body = RigidBodyDesc::new()
        .translation(position)
        .rotation(rotation)
        .status(body_status)
        .mass(1.2)
        // .gravity_enabled(true)
        // .set_status(BodyStatus::Kinematic)
        .velocity(Velocity::linear(velocity.x, velocity.y))
        .build();

    let body_handle = self.body_set.insert(rigid_body);

    let shape = thing.to_shape_handle();
    let collider = ColliderDesc::new(shape)
        // .translation(position)
        .density(1.0)
        .material(MaterialHandle::new(BasicMaterial::new(1.0, 0.8)))
        .build(BodyPartHandle(body_handle, 0));

    let collider_handle = self.collider_set.insert(collider);

    // let foo: f32 = body_handle;
    let ft = FizThing {
      // fiz: self,
      // OPT no clone
      thing: thing.clone(),
      body_handle: body_handle,
      collider_handle: collider_handle,
    };
    // rigid_body.set_user_data(Some(Box::new(ft)));
    return ft;
  }

  pub fn current(&self, ft: &FizThing) -> Thing {
    let body = self.body_set.rigid_body(ft.body_handle).unwrap();

    // TODO always making copies
    let pos = body.position().translation;
    let rot: f32 = body.position().rotation.angle();
    let tr3 = Isometry3::new(Vector3::new(pos.x, pos.y, 0.), Vector3::z() * rot);
    // OPT no clone
    let thing_current = transform(&ft.thing, tr3);
    return thing_current;
  }

  pub fn step(&mut self) {
    self.mechanical_world.step(
        &mut self.geometrical_world,
        &mut self.body_set,
        &mut self.collider_set,
        &mut self.joint_constraints,
        &mut self.force_generators
    );
    for contact in self.geometrical_world.contact_events() {
      // Handle contact events.
      // let foo: f32 = contact;
      println!("HEY {:?}", contact);
      // match contact {
      //    Started(h0, h1) => {
      //      // let foo: f32 = h0;
      //      let rbh0 = self.collider_set.get(*h0).unwrap().body();
      //      let rb0: u32 = self.body_set.get(rbh0).unwrap();
      //    }
      //  }
    }
  }
}
