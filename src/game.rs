use macroquad::prelude::*;
use na::{Vector3, Isometry3};
use std::process::exit;

use crate::fiz::Fiz;
// use crate::thing::lines;
use crate::thing::make_cube;
use crate::thing::transform;

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
    loop {
        clear_background(LIGHTGRAY);

        // Going 3d!

        let x = 10.;
        let y = 7.;
        let ang = get_time() * 1.;
        let c = ang.cos();
        let s = ang.sin();
        let rx = x * c + y * s;
        let ry = y * c - x * s;

        set_camera(&Camera3D {
            // position: vec3(-20., 15., 0.),
            position: vec3(rx as f32, 6., ry as f32),
            // position: vec3(0.8, 6., 8.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        let cube = make_cube();
        let axisangle = Vector3::y() * std::f32::consts::FRAC_PI_4;
        let tr = Isometry3::new(Vector3::new(0., 0., 3.), axisangle);
        let cube2 = transform(cube, tr);
        crate::thing::lines::draw_thing(cube2);

        self.fiz.main();

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
