extern crate nalgebra as na;
use na::{Matrix4, Point4};

use crate::mesh_data::MeshData;

#[derive(Clone)]
pub struct Transform {
    pub base: Matrix4<f64>,
    pub mesh: Option<MeshData>,
    pub children: Vec<Transform>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            #[rustfmt::skip]
            base: Matrix4::new(
                         1.0, 0.0, 0.0, 0.0,
                         0.0, 1.0, 0.0, 0.0,
                         0.0, 0.0, 1.0, 0.0,
                         0.0, 0.0, 0.0, 1.0),
            mesh: None,
            children: vec![],
        }
    }

    pub fn new_translation(x: f64, y: f64, z: f64) -> Self {
        Self {
            #[rustfmt::skip]
            base: Matrix4::new(
                         1.0, 0.0, 0.0, x,
                         0.0, 1.0, 0.0, y,
                         0.0, 0.0, 1.0, z,
                         0.0, 0.0, 0.0, 1.0),

            mesh: None,
            children: vec![],
        }
    }

    pub fn setup_default_cube(&mut self) {
        let verts: Vec<Point4<f64>> = vec![
            Point4::new(1.000000, 1.000000, -1.000000, 1.0),
            Point4::new(1.000000, -1.000000, -1.000000, 1.0),
            Point4::new(1.000000, 1.000000, 1.000000, 1.0),
            Point4::new(1.000000, -1.000000, 1.000000, 1.0),
            Point4::new(-1.000000, 1.000000, -1.000000, 1.0),
            Point4::new(-1.000000, -1.00000, -1.000000, 1.0),
            Point4::new(-1.000000, 1.000000, 1.000000, 1.0),
            Point4::new(-1.000000, -1.00000, 1.000000, 1.0),
        ];
        Transform::translate(self, 2.0, 0.0, 0.0);

        self.mesh = Some(MeshData { verts })
    }

    fn get_translation_matrix(x: f64, y: f64, z: f64) -> Matrix4<f64> {
        #[rustfmt::skip]
        return Matrix4::new(
          1.0, 0.0, 0.0, x,
          0.0, 1.0, 0.0, y,
          0.0, 0.0, 1.0, z,
          0.0, 0.0, 0.0, 1.0);
    }

    pub fn translate_self(&mut self, x: f64, y: f64, z: f64) {
        Transform::translate(self, x, y, z)
    }

    pub fn rotate_self(&mut self, x: f64, y: f64, z: f64) {
        Transform::rotate_x(self, x);
        Transform::rotate_y(self, y);
        Transform::rotate_z(self, z);
    }

    pub fn translate(transform: &mut Transform, x: f64, y: f64, z: f64) {
        #[rustfmt::skip]
        let matrix = Transform::get_translation_matrix(x, y, z);
        transform.base = transform.base * matrix;
    }

    pub fn rotate_y(transform: &mut Transform, angle: f64) {
        let angle_rad = angle.to_radians();
        #[rustfmt::skip]
        let matrix: Matrix4<f64> =
          Matrix4::new(
                   angle_rad.cos().into(),           0.0, angle_rad.sin().into(),   0.0 ,
                   0.0,                              1.0, 0.0,                      0.0,
                   (angle_rad.sin() * -1.0).into(),  0.0, angle_rad.cos().into(),   0.0,
                   0.0,                              0.0, 0.0,                      1.0);
        transform.base = transform.base * matrix;
    }

    pub fn rotate_x(transform: &mut Transform, angle: f64) {
        let angle_rad = angle.to_radians();
        #[rustfmt::skip]
        let matrix: Matrix4<f64> =
          Matrix4::new(
                   1.0, 0.0,                    0.0,                             0.0,
                   0.0, angle_rad.cos().into(), (angle_rad.sin() * -1.0).into(), 0.0,
                   0.0, angle_rad.sin().into(), angle_rad.cos().into(),          0.0,
                   0.0, 0.0,                    0.0,                             1.0);
        transform.base = transform.base * matrix;
    }

    pub fn rotate_z(transform: &mut Transform, angle: f64) {
        let angle_rad = angle.to_radians();
        #[rustfmt::skip]
        let matrix: Matrix4<f64> =
          Matrix4::new(
                   angle_rad.cos().into(), (angle_rad.sin() * -1.0).into(), 0.0, 0.0,
                   angle_rad.sin().into(), angle_rad.cos().into(),          0.0, 0.0,
                   0.0,                    0.0,                             1.0, 0.0,
                   0.0,                    0.0,                             0.0, 1.0);

        transform.base = transform.base * matrix;
    }
}
