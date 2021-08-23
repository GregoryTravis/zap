extern crate nalgebra as na;

use macroquad::prelude::*;
use core::f64::consts::PI;
//use na::{Vector3, Rotation3};
use na::{Vector3, Isometry3};
use std::process::exit;

mod thing;
mod fiz;
// use crate::thing::thing::Thing;
use crate::thing::thing::make_cube;
use crate::thing::thing::transform;
// use crate::make_cube;
// mod thing_lines;
// use crate::thing;
// use crate::thing_lines;

// mod thing;
// mod thing_lines;

#[macroquad::main("3D")]
async fn main() {
    // let rust_logo = load_texture("examples/rust.png").await.unwrap();
    // let ferris = load_texture("examples/ferris.png").await.unwrap();
    // let axis  = Vector3::x_axis();
    // let angle = 1.57;
    // let b     = Rotation3::from_axis_angle(&axis, angle);

    #[derive(Clone, Copy)]
    struct Cube {
      pos: Vector3<f32>,
      sz: f32,
      color: Color,
      // speed: f64,
    }

    const NUM_CUBES: usize = 25000;
    // let cubes = [Cube; NUM_CUBES];
    let mut cubes = Vec::with_capacity(NUM_CUBES);
    let max_z = 10.;
    let max_rad = 5.;
    for _i in 0..NUM_CUBES {
        let z = rand::gen_range(-max_z, max_z);
        let z_plane_ang = rand::gen_range(0., 2.*PI);
        let cs : f64 = z_plane_ang.cos();
        let sn : f64 = z_plane_ang.sin();
        let rad = rand::gen_range(-max_rad, max_rad);
        let x : f32 = (cs * rad) as f32;
        let y : f32 = (sn * rad) as f32;
        let sz = rand::gen_range(-1., 1.);
        let color = Color::new(rand::gen_range(0., 1.), rand::gen_range(0., 1.), rand::gen_range(0., 1.), 1.);
        // let speed = rand::gen_range(0., 2.*PI);
        let cube = Cube {
          pos: Vector3::new(x, y, z),
          sz: sz,
          color: color,
          // speed: speed,
        };
        cubes.push(cube);
    }

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

        // draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
        // draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), DARKBLUE);
        // draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), YELLOW);

        // draw_plane(vec3(-8., 0., -8.), vec2(5., 5.), ferris, WHITE);

        // draw_cube(vec3(-5., 1., -2.), vec3(2., 2., 2.), rust_logo, WHITE);
        // draw_cube(vec3(-5., 1., 2.), vec3(2., 2., 2.), ferris, WHITE);
        // draw_cube(vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);

        // draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);

        // let mut rng = rand::thread_rng();

        // let num_cubes = 20;
        // let max_z = 10.;
        // let max_rad = 7.;
        // for _i in 0..num_cubes {
        //     let z = rand::gen_range(-max_z, max_z);
        //     let z_plane_ang = rand::gen_range(0., 2.*PI);
        //     let cs : f64 = z_plane_ang.cos();
        //     let sn : f64 = z_plane_ang.sin();
        //     let rad = rand::gen_range(-max_rad, max_rad);
        //     let x : f32 = (cs * rad) as f32;
        //     let y : f32 = (sn * rad) as f32;
        //     let cr = rand::gen_range(-3., 3.);
        //     draw_cube_wires(vec3(x, y, z), vec3(cr, cr, cr), DARKGREEN);
        // }

        // for cube in &cubes {
        //     let sz = cube.sz;
        //     let pos = cube.pos;

        //     // let axis  = Vector3::x_axis();
        //     // let angle = get_time() * 1. * cube.speed;
        //     // let _m     = Rotation3::from_axis_angle(&axis, angle);

        //     let v3 = vec3(pos.x, pos.y, pos.z);
        //     draw_cube_wires(v3, vec3(sz, sz, sz), cube.color);
        // }

        let cube = make_cube();
        // let rot = Rotation3::new(Vector3::z() * std::f32::consts::FRAC_PI_4);
        // let rot = Rotation3::from_axis_angle(&Vector3::y_axis(), std::f32::consts::FRAC_PI_4);
        // let tr = Isometry3::from_parts(Vector3::new(0, 0, 0), Vector3::y() * std::f32::consts::FRAC_PI_4);
        let axisangle = Vector3::y() * std::f32::consts::FRAC_PI_4;
        let tr = Isometry3::new(Vector3::new(0., 0., 3.), axisangle);
        let cube2 = transform(cube, tr);
        thing::lines::draw_thing(cube2);

        fiz::fiz_main();

        // Back to screen space, render some text

        set_default_camera();
        draw_text(&format!("WELCOME TO 3D WORLD (fps {})", get_fps()), 10.0, 20.0, 30.0, BLACK);
        // if is_key_down(KeyCode::W) {
        //   println!("ooo");
        // }
        // if is_key_down(KeyCode::Space) {
        //   println!("ooo {:?}", get_fps());
        // }
        if is_key_down(KeyCode::LeftSuper) && is_key_down(KeyCode::Q) {
            exit(0);
        }
        // let a = get_last_key_pressed();
        // println!("ooo {:?} {:?}", a, get_fps());

        next_frame().await
    }
}
