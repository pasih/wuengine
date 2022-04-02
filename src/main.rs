use obj_reader::read_object;
use pixels::{Error, Pixels, SurfaceTexture};
use transform::Transform;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

extern crate nalgebra as na;

mod mesh_data;
mod obj_reader;
mod scene;
mod transform;

use crate::scene::Scene;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> Result<(), Error> {
    let md = read_object("../cube/monkey.obj").expect("Cannot read file");
    let mut main_obj = Transform::new();
    main_obj.mesh = Some(md);

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut scene = Scene::new();
    scene.set_default_camera_position();
    scene.add_object(&main_obj);

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);

        WindowBuilder::new()
            .with_title("WUEngine")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            scene.draw(pixels.get_frame(), WIDTH as usize, HEIGHT as usize);

            if pixels
                .render()
                .map_err(|e| println!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::A) {
                let obj = &mut scene.objects[0];
                obj.translate_self(1.0, 0.0, 0.0)
            }

            if input.key_pressed(VirtualKeyCode::Q) {
                let obj = &mut scene.objects[0];
                obj.translate_self(0.0, 1.0, 0.0)
            }

            if input.key_pressed(VirtualKeyCode::E) {
                let obj = &mut scene.objects[0];
                obj.translate_self(0.0, -1.0, 0.0)
            }

            if input.key_pressed(VirtualKeyCode::D) {
                let obj = &mut scene.objects[0];
                obj.translate_self(-1.0, 0.0, 0.0)
            }

            if input.key_pressed(VirtualKeyCode::S) {
                let obj = &mut scene.objects[0];
                obj.translate_self(0.0, 0.0, 1.0)
            }

            if input.key_pressed(VirtualKeyCode::W) {
                let obj = &mut scene.objects[0];
                obj.translate_self(0.0, 0.0, -1.0)
            }

            if input.key_pressed(VirtualKeyCode::W) {
                let obj = &mut scene.objects[0];
                obj.translate_self(0.0, 0.0, -1.0)
            }

            if input.key_pressed(VirtualKeyCode::T) {
                println!("Rotating");
                let obj = &mut scene.objects[0];
                obj.rotate_self(10.0, 0.0, 0.0)
            }

            if input.key_pressed(VirtualKeyCode::T) {
                let obj = &mut scene.objects[0];
                obj.rotate_self(10.0, 0.0, 0.0)
            }

            if input.key_pressed(VirtualKeyCode::G) {
                let obj = &mut scene.objects[0];
                obj.rotate_self(-10.0, 0.0, 0.0)
            }

            if input.key_pressed(VirtualKeyCode::Y) {
                let obj = &mut scene.objects[0];
                obj.rotate_self(0.0, 10.0, 0.0)
            }

            if input.key_pressed(VirtualKeyCode::H) {
                let obj = &mut scene.objects[0];
                obj.rotate_self(0.0, -10.0, 0.0)
            }

            if input.key_pressed(VirtualKeyCode::U) {
                let obj = &mut scene.objects[0];
                obj.rotate_self(0.0, 0.0, 10.0)
            }

            if input.key_pressed(VirtualKeyCode::J) {
                let obj = &mut scene.objects[0];
                obj.rotate_self(0.0, 0.0, -10.0)
            }

            window.request_redraw();
        }
    })
}
