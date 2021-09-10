use macroquad::prelude::*;
use na::{Vector2};
use std::process::exit;

use crate::fiz::Fiz;
// use crate::thing::lines;
use crate::thing::make_cube;
use crate::thing::make_cuboid;
// use crate::thing::transform;
use crate::thing::make_grid_xy;

pub struct Game {
  pub fiz: Fiz,
}

impl Game {
  pub fn new() -> Game {
    return Game {
      fiz: Fiz::new(),
    };
  }

  pub async fn main(&mut self) {
    let mut fiz_things = Vec::new();

    let grid = make_grid_xy(5);

    let cube = make_cube();
    // let axisangle = Vector3::y() * std::f32::consts::FRAC_PI_4;
    // let tr = Isometry3::new(Vector3::new(0., 0., 3.), axisangle);
    // let cube2 = transform(cube, tr);

    #[allow(unused_variables)] // TODO remove
    // let rot = 0.9*3.14159265*0.25:
    let rot = 0.0;
    for i in 0..=2 {
      println!("UUU {}", i);
      let f = (i-1) as f32;
      let cube_ft = self.fiz.add_thing(&cube, Vector2::new(0.0, f), rot, Vector2::new(15.0, 0.0), false);
      fiz_things.push(cube_ft);
    }

    // wall
    // let ground_size = r!(5.0);
    let wall_cuboid = make_cuboid(1.0, 10.0, 1.0);
    fiz_things.push(
      self.fiz.add_thing(&wall_cuboid, Vector2::new(10.0, 0.), 0.0, Vector2::new(0.0, 0.0), true));
    fiz_things.push(
      self.fiz.add_thing(&wall_cuboid, Vector2::new(-10.0, 0.), 0.0, Vector2::new(0.0, 0.0), true));
    fiz_things.push(
      self.fiz.add_thing(&wall_cuboid, Vector2::new(0.0, 10.), std::f32::consts::FRAC_PI_2, Vector2::new(0.0, 0.0), true));
    fiz_things.push(
      self.fiz.add_thing(&wall_cuboid, Vector2::new(0.0, -10.), std::f32::consts::FRAC_PI_2, Vector2::new(0.0, 0.0), true));

    loop {
        clear_background(LIGHTGRAY);

        // Going 3d!

        let _x = 10.;
        let _y = 7.;
        // let ang = get_time() * 1.;
        let ang = core::f32::consts::PI/1.0;
        // let ang: f32 = 0.0;
        // let ang: f32 = 0.0; // 1.8;
        let _c = ang.cos();
        let _s = ang.sin();
        // let rx = x * c + y * s;
        // let ry = y * c - x * s;

        set_camera(&Camera3D {
            // position: vec3(-20., 15., 0.),
            // position: vec3(rx as f32, 36., ry as f32),
            // position: vec3(0., 0., 10.0),
            position: vec3(1., -8., 20.0),
            // position: vec3(0.8, 6., 8.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        // draw_grid(20, 1., BLACK, GRAY);

        self.fiz.step();

        for ft in &fiz_things {
          let current = self.fiz.current(&ft);
          crate::thing::lines::draw_thing(&current);
        }

        crate::thing::lines::draw_thing(&grid);

        // Back to screen space, render some text

        set_default_camera();

        draw_text(&format!("WELCOME TO 3D WORLD (fps {})", get_fps()), 10.0, 20.0, 30.0, BLACK);

        if is_key_down(KeyCode::LeftSuper) && is_key_down(KeyCode::Q) {
            exit(0);
        }

        next_frame().await
    }
  }
}
