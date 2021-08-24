// use na::{Vector3};
use macroquad::prelude::*;
// mod thing_lines;
use crate::thing::Thing;

pub fn draw_thing(th: Thing) {
    for line in th.lines {
        let (i0, i1) = line;
        let pt0 = th.points[i0];
        let pt1 = th.points[i1];
        let vec0 = vec3(pt0.x, pt0.y, pt0.z);
        let vec1 = vec3(pt1.x, pt1.y, pt1.z);
        // println!("line {} {} {} {}", i0, i1, pt0, pt1);
        draw_line_3d(vec0, vec1, DARKGREEN);
    }
}
