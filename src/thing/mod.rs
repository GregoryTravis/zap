pub mod lines;

use na::{Point3, Isometry3, Point2};
use ncollide2d::shape::{ShapeHandle, ConvexPolygon};

// pub mod thing;

// OPT: don't clone em all the time
#[derive(Clone)]
pub struct Thing {
    pub points: Vec<Point3<f32>>,
    pub lines: Vec<(usize, usize)>,
}

pub fn transform(t: Thing, tr: Isometry3<f32>) -> Thing {
    let mut new_points = Vec::with_capacity(t.points.len());
    for p in t.points {
        new_points.push(tr * p);
    }
    return Thing {
        points: new_points,
        lines: t.lines,
    };
}

pub fn make_cube() -> Thing {
  return make_cuboid(1.0, 1.0, 1.0);
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
