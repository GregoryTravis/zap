// use na::{Vector3};
use na::{Point3};
use macroquad::prelude::*;
// mod thing_lines;
use crate::thing::Thing;
// use std::convert::TryInto;

pub fn draw_thing(th: &Thing) {
    for line in &th.lines {
        let (i0, i1) = line;
        let ui0 = *i0 as usize;
        let ui1 = *i1 as usize;
        // let pt0: Point3<f32> = th.points[i0.try_into().unwrap()];
        // let pt1: Point3<f32> = th.points[i1.try_into().unwrap()];
        let pt0: Point3<f32> = th.points[ui0];
        let pt1: Point3<f32> = th.points[ui1];
        let vec0 = vec3(pt0.x, pt0.y, pt0.z);
        let vec1 = vec3(pt1.x, pt1.y, pt1.z);
        // println!("line {} {} {} {}", i0, i1, pt0, pt1);
        draw_line_3d(vec0, vec1, DARKGREEN);
    }
}
