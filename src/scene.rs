extern crate nalgebra as na;
use na::{Matrix4, Point4};

use crate::transform::Transform;

fn _ortho_matrix(w: f64, h: f64) -> Matrix4<f64> {
    let far = 1.0;
    let near = 0.0;
    let f1 = 2.0 / (far - near);
    let f2 = (far + near) / (far - near);

    #[rustfmt::skip]
    return Matrix4::new(
        1.0 / w, 0.0,     0.0, 0.0,
        0.0,     1.0 / h, 0.0, 0.0,
        0.0,     0.0,     f1,  f2,
        0.0,     0.0,     0.0, 1.0
    );
}

// Check that far / near are calculated correctly (correct vector)
fn perspective_matrix(angle: f64, w: f64, h: f64) -> Matrix4<f64> {
    let fov = 1.0 / f64::tan(angle.to_radians() / 2.0);
    let aspect = w / h;
    let far = 3.0;
    let near = 0.5;

    #[rustfmt::skip]
    return Matrix4::new(
        fov * aspect, 0.0,     0.0,                                 0.0,
        0.0,          fov,     0.0,                                 0.0,
        0.0,          0.0,     (far + near) / (far - near),         1.0,
        0.0,          0.0,     (2.0 * near * far) /( near-far),     0.0
    );
}

pub struct Scene {
    pub camera: Transform,
    pub objects: Vec<Transform>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: Transform::new(),
            objects: vec![],
        }
    }

    pub fn add_test_objects(&mut self) {
        let mut cubed = Transform::new_translation(0.5, 9.0, 0.2);
        cubed.setup_default_cube();
        self.objects.push(cubed);
    }

    pub fn add_object(&mut self, transform: &Transform) {
        self.objects.push(transform.to_owned())
    }

    pub fn set_default_camera_position(&mut self) {
        Transform::translate(&mut self.camera, 0.0, -2.0, 0.0);
        Transform::rotate_x(&mut self.camera, 90.0);
    }

    // TODO: Move the conversion out of scene
    fn local_to_world(&self, t: Transform, p: Point4<f64>) -> Point4<f64> {
        t.base * p
    }

    fn world_to_camera(&self, p: Point4<f64>) -> Point4<f64> {
        self.camera.base * p
    }

    fn camera_to_screen(&self, w: f64, h: f64, p: Point4<f64>) -> Option<(f64, f64)> {
        let transformed = perspective_matrix(60.0, w, h) * p;

        // TODO: Check clipping
        // if ...
        // {
        //     return None;
        // }

        let half_w = 0.5 * w;
        let half_h = 0.5 * h;
        let x = (transformed.x * w) / (2.0 * transformed.w) + half_w;
        let y = (transformed.y * h) / (2.0 * transformed.w) + half_h;
        Some((x, y))
    }

    fn render_objects(&self, w: u64, h: u64) -> [[u8; 4]; 600 * 800] {
        let mut buf = [[0u8; 4]; 600 * 800];

        for obj in &self.objects {
            match &obj.mesh {
                Some(md) => {
                    for vert in &md.verts {
                        let wc = self.local_to_world(obj.to_owned(), *vert);
                        let cc = self.world_to_camera(wc);
                        let xy_opt = self.camera_to_screen(w as f64, h as f64, cc);

                        match xy_opt {
                            Some(xy) => {
                                let idx = xy.0.floor() + w as f64 * xy.1.floor();

                                // TODO: remove hack check once there's clipping
                                if idx < 480000.0 {
                                    buf[idx as usize] = [0xff, 0x00, 0xff, 0xff];
                                }
                            }
                            None => (),
                        }
                    }
                }
                _ => (),
            }
        }

        buf
    }

    pub fn draw(&self, frame: &mut [u8], w: usize, h: usize) {
        let buf = self.render_objects(w as u64, h as u64);

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let rgba = buf[i];
            pixel.copy_from_slice(&rgba);
        }
    }
}
