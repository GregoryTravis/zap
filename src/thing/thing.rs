use na::{Vector3, Point3, Rotation3, Isometry3};

// pub mod thing;

// #[derive(Clone, Copy)]
pub struct Thing {
    pub points: Vec<Point3<f32>>,
    pub lines: Vec<(usize, usize)>,
}

pub fn transform(t: Thing, tr: Isometry3<f32>) -> Thing {
    let mut newPoints = Vec::with_capacity(t.points.len());
    for p in t.points {
        newPoints.push(tr * p);
    }
    return Thing {
        points: newPoints,
        lines: t.lines,
    };
}

pub fn make_cube() -> Thing {
  return Thing {
      points: vec![
          // Vector3::new( 0.5, 0.5, 0.5),
          // Vector3::new(-0.5, 0.5, 0.5),
          // Vector3::new(-0.5,-0.5, 0.5),
          // Vector3::new( 0.5,-0.5, 0.5),
          // Vector3::new( 0.5,-0.5,-0.5),
          // Vector3::new(-0.5,-0.5,-0.5),
          // Vector3::new(-0.5, 0.5,-0.5),
          // Vector3::new( 0.5, 0.5,-0.5),
          Point3::new( 3.5, 0.5, 0.5),
          Point3::new(-0.5, 0.5, 0.5),
          Point3::new(-0.5,-0.5, 0.5),
          Point3::new( 3.5,-0.5, 0.5),
          Point3::new( 3.5,-0.5,-0.5),
          Point3::new(-0.5,-0.5,-0.5),
          Point3::new(-0.5, 0.5,-0.5),
          Point3::new( 3.5, 0.5,-0.5),
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
