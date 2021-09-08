pub mod lines;

use na::{Point3, Isometry3, Point2};
use ncollide2d::shape::{ShapeHandle, ConvexPolygon};
use std::convert::TryInto;

// pub mod thing;

// OPT: don't clone em all the time
#[derive(Clone)]
pub struct Thing {
    pub points: Vec<Point3<f32>>,
    pub lines: Vec<(usize, usize)>,
}

pub fn transform(t: &Thing, tr: Isometry3<f32>) -> Thing {
    let mut new_points = Vec::with_capacity(t.points.len());
    for p in &t.points {
        new_points.push(tr * p);
    }
    return Thing {
        points: new_points,
        lines: t.lines.clone(),
    };
}

pub fn make_thing_from_point_pairs(pps: &Vec<(Point3<f32>, Point3<f32>)>) -> Thing {
  let mut points: Vec<Point3<f32>> = Vec::with_capacity(pps.len() * 2);
  let mut lines: Vec<(usize, usize)> = Vec::with_capacity(pps.len());
  for (i, (a, b)) in pps.iter().enumerate() {
    points.push(*a);
    points.push(*b);
    lines.push((i * 2, i * 2 + 1));
  }
  return Thing {
    points: points,
    lines: lines,
  };
}

pub fn make_cube() -> Thing {
  return make_cuboid(0.5, 0.5, 0.5);
}

pub fn make_cuboid(x: f32, y: f32, z: f32) -> Thing {
  return Thing {
      points: vec![
          Point3::new( x,  y,  z),
          Point3::new(-x,  y,  z),
          Point3::new(-x, -y,  z),
          Point3::new( x, -y,  z),
          Point3::new( x, -y, -z),
          Point3::new(-x, -y, -z),
          Point3::new(-x,  y, -z),
          Point3::new( x,  y, -z),
      ],
      lines: vec![
          (0, 1),
          (1, 2),
          (2, 3),
          (3, 0),
          (4, 5),
          (5, 6),
          (6, 7),
          (7, 4),
          (3, 4),
          (2, 5),
          (1, 6),
          (0, 7),
      ],
    };
}

pub fn make_grid_xy(half_size: u32) -> Thing {
  let c = half_size * 2 + 1;
  let hs = half_size as i32;
  let fhs = half_size as f32;
  let mut pps: Vec<(Point3<f32>, Point3<f32>)> = Vec::with_capacity((c*2).try_into().unwrap());
  for i in -hs..=hs {
    let fi = i as f32;
    pps.push((Point3::new(fi, -fhs, 0.), Point3::new(fi, fhs, 0.)));
    pps.push((Point3::new(-fhs, fi, 0.), Point3::new(fhs, fi, 0.)));
  };
  return make_thing_from_point_pairs(&pps);
}

impl Thing {
  pub fn points2d(&self) -> Vec<Point2<f32>> {
    return self.points.iter().map(|&v3| Point2::new(v3.x, v3.y)).collect::<Vec<_>>();
  }

  pub fn to_shape_handle(&self) -> ShapeHandle<f32> {
    let pts2 = self.points2d();
    let ch = ConvexPolygon::try_from_points(&pts2).expect("convex hull");
    let shape = ShapeHandle::new(ch);
    return shape;
  }
}
