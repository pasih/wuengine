use na::Point4;

extern crate nalgebra as na;

#[derive(Clone)]
pub struct MeshData {
    pub verts: Vec<Point4<f64>>,
}
