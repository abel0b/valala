#![macro_use]
extern crate glium;
extern crate image;
extern crate cgmath;

use std::env;

// use glium::{glutin, Surface};
// use std::io::Cursor;
// use glium::{uniform, Surface};
// use std::f32::consts::PI;
//
// use std::time::Duration;
// use std::time::Instant;
//
// use std::collections::HashMap;

use std::error::Error;
use std::result::Result;
use std::boxed::Box;

mod map;
mod hex;
mod game;
mod camera;
mod resource;
mod state;
mod vertex;
mod picking;
mod identifier;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Valala v{}", env!("CARGO_PKG_VERSION"));
    let mut game = game::Game::new()?;
    game.start();
    Ok(())

    // let mut event_loop = glium::glutin::EventsLoop::new();
    //
    // let wb = glium::glutin::WindowBuilder::new();
    // let cb = glium::glutin::ContextBuilder::new().with_depth_buffer(24).with_multisampling(4);
    // let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    //
    // let mut closed = false;
    //
    // let image = image::load(Cursor::new(&include_bytes!("../res/grass.png")[..]), image::PNG).unwrap().to_rgba();
    // let image_dimensions = image.dimensions();
    // let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    //
    // let grass_texture = glium::texture::Texture2d::new(&display, image).unwrap();
    //
    // // let image = image::load(Cursor::new(&include_bytes!("../res/dirt.png")[..]), image::PNG).unwrap().to_rgba();
    // // let image_dimensions = image.dimensions();
    // // let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    // // let dirt_texture = glium::texture::Texture2d::new(&display, image).unwrap();
    // let vertex_shader_src = r#"
    // #version 140
    //
    // in uint id;
    // in vec3 position;
    // in vec2 tex_coords;
    // uniform vec2 coordinates;
    // uniform mat4 model;
    // uniform mat4 perspective;
    // uniform mat4 view;
    //
    // out vec2 v_tex_coords;
    // out vec2 v_coordinates;
    //
    // void main() {
    //     v_tex_coords = tex_coords;
    //     v_coordinates = coordinates;
    //     gl_Position =  perspective * view * model * vec4(position, 1.0);
    // }
    // "#;
    //
    // let fragment_shader_src = r#"
    // #version 140
    //
    // in vec2 v_tex_coords;
    // uniform sampler2D tex;
    // in vec2 v_coordinates;
    //
    // out vec4 color;
    //
    // float modI(float a,float b) {
    //     float m=a-floor((a+0.5)/b)*b;
    //     return floor(m+0.5);
    // }
    //
    // void main() {
    //     if (modI(v_coordinates[0] - v_coordinates[1], 3) == 0.0) {
    //         color = texture(tex, v_tex_coords) - vec4(0.1,0.1,0.1,0.0);
    //     }
    //     else if  (modI(v_coordinates[0] - v_coordinates[1], 3) == 1.0) {
    //         color = texture(tex, v_tex_coords) - vec4(0.05,0.05,0.05,0.0);
    //     }
    //     else {
    //         color = texture(tex, v_tex_coords);
    //     }
    // }
    // "#;
    // let select_vertex_shader_src = r#"
    // #version 140
    //
    // in uint id;
    // in vec3 position;
    // uniform mat4 model;
    // uniform mat4 perspective;
    // uniform mat4 view;
    //
    // void main() {
    //     gl_Position =  perspective * view * model * vec4(position, 1.0);
    // }
    // "#;
    //
    // let select_fragment_shader_src = r#"
    // #version 140
    //
    // out vec4 f_id;
    //
    // void main() {
    //     f_id = vec4(0.65, 0.12, 0.17, 1.0);
    // }
    // "#;
    //
    // let picking_vertex_shader_src = r#"
    // #version 140
    //
    // in uint id;
    // in vec3 position;
    // uniform mat4 model;
    // uniform mat4 perspective;
    // uniform mat4 view;
    //
    // flat out uint v_id;
    //
    // void main() {
    //     v_id = id;
    //     gl_Position =  perspective * view * model * vec4(position, 1.0);
    // }
    // "#;
    //
    // let picking_fragment_shader_src = r#"
    // #version 140
    //
    // flat in uint v_id;
    //
    // out uint f_id;
    //
    // void main() {
    //     f_id = v_id;
    // }
    // "#;
    //
    //
    // let vertex_shader_grid_src = r#"
    // #version 140
    //
    // in vec3 position;
    // uniform mat4 model;
    // uniform mat4 view;
    // uniform mat4 perspective;
    //
    // void main() {
    //     gl_Position =  perspective * view * model * vec4(position, 1.0);
    // }
    // "#;
    //
    // let fragment_shader_grid_src = r#"
    // #version 140
    //
    // out vec4 color;
    //
    // void main() {
    //     color = vec4(0.1, 0.1, 0.1, 1.0);
    // }
    // "#;
    //
    // let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    // let grid_program = glium::Program::from_source(&display, vertex_shader_grid_src, fragment_shader_grid_src, None).unwrap();
    // let picking_program = glium::Program::from_source(&display, picking_vertex_shader_src, picking_fragment_shader_src, None).unwrap();
    // let select_program = glium::Program::from_source(&display, select_vertex_shader_src, select_fragment_shader_src, None).unwrap();
    //
    // let mut grid: HashMap<u32, grid::Hex> = HashMap::new();
    //
    // let mut i = 1;
    // let map_radius: i32 = 3;
    //
    // for q in -map_radius..map_radius+1 {
    //     let r1 = std::cmp::max(-map_radius, -q - map_radius);
    //     let r2 = std::cmp::min(map_radius, -q + map_radius);
    //     for r in r1..r2+1 {
    //         grid.insert(i, grid::Hex::new(i, &display, q,r));
    //         i = i+1;
    //     }
    // }
    //
    // let camera: cgmath::Matrix4<f32> = cgmath::Matrix4::look_at_dir(
    //     cgmath::Point3 { x: 0.0, y: 0.0, z: 0.0 },
    //     cgmath::Vector3 { x: 0.0, y: 0.0, z: 1.0 },
    //     cgmath::Vector3 { x: 0.0, y: 1.0, z: 0.0 },
    // );
    // let view: [[f32; 4]; 4] = cgmath::conv::array4x4(camera);
    //
    // let params = glium::DrawParameters {
    //     depth: glium::Depth {
    //         test: glium::DepthTest::IfLess,
    //         write: true,
    //         .. Default::default()
    //     },
    //     .. Default::default()
    // };
    //
    // let mut cursor_position: Option<(i32, i32)> = None;
    //
    // let mut picking_attachments: Option<(glium::texture::UnsignedTexture2d, glium::framebuffer::DepthRenderBuffer)> = None;
    // let picking_pbo: glium::texture::pixel_buffer::PixelBuffer<u32>
    //     = glium::texture::pixel_buffer::PixelBuffer::new_empty(&display, 1);
    //
    // // let mut accumulator = Duration::new(0, 0);
    // let mut previous_clock = Instant::now();
    //
    // while !closed {
    //     let picked_object = {
    //         let data = picking_pbo.read().map(|d| d[0]).unwrap_or(0);
    //         if data != 0 {
    //             Some(data)
    //         } else {
    //             None
    //         }
    //     };
    //
    //     let mut target = display.draw();
    //     target.clear_color_and_depth((0.9, 0.9, 0.9, 1.0), 1.0);
    //
    //     let perspective: [[f32; 4]; 4] = {
    //         let (width, height) = target.get_dimensions();
    //         let aspect_ratio = height as f32 / width as f32;
    //         let right: f32 = 10.0;
    //         let left: f32 = -10.0;
    //         let bottom: f32 = -10.0;
    //         let top: f32 = 10.0;
    //         let far: f32 = 100.0;
    //         let near: f32 = -100.0;
    //
    //         cgmath::conv::array4x4(
    //             cgmath::Matrix4::from_nonuniform_scale(aspect_ratio, 1.0, 1.0) * cgmath::ortho(left, right, bottom, top, near, far)
    //         )
    //     };
    //
    //     let model_mat = cgmath::Matrix4::from_angle_x(cgmath::Rad(-(1.0/2.0f32.sqrt()).atan())) * cgmath::Matrix4::from_angle_y(cgmath::Rad(PI/4.0));
    //     // let inv_model_mat = cgmath::Matrix4::from_angle_y(cgmath::Rad(-PI/4.0)) * cgmath::Matrix4::from_angle_x(cgmath::Rad((1.0/2.0f32.sqrt()).atan()));
    //
    //     let model: [[f32; 4]; 4] = cgmath::conv::array4x4(model_mat);
    //
    //     let uniforms = uniform! {
    //         view: view,
    //         perspective: perspective,
    //         model: model,
    //     };
    //
    //     if picking_attachments.is_none() || (
    //         picking_attachments.as_ref().unwrap().0.get_width(),
    //         picking_attachments.as_ref().unwrap().0.get_height().unwrap()
    //     ) != target.get_dimensions() {
    //         let (width, height) = target.get_dimensions();
    //         picking_attachments = Some((
    //             glium::texture::UnsignedTexture2d::empty_with_format(
    //                 &display,
    //                 glium::texture::UncompressedUintFormat::U32,
    //                 glium::texture::MipmapsOption::NoMipmap,
    //                 width, height,
    //             ).unwrap(),
    //             glium::framebuffer::DepthRenderBuffer::new(
    //                 &display,
    //                 glium::texture::DepthFormat::F32,
    //                 width, height,
    //             ).unwrap()
    //         ))
    //     }
    //
    //     for i in 1..grid.len()+1 {
    //         let uniforms_texture = uniform! {
    //             tex: &grass_texture,
    //             coordinates: (grid[&(i as u32)].coordinates.0 as f32,  grid[&(i as u32)].coordinates.1 as f32),
    //             view: view,
    //             model: model,
    //             perspective: perspective,
    //         };
    //         target.draw(&grid[&(i as u32)].vertices_buffer, &grid[&(i as u32)].indices_buffer, &program, &uniforms_texture, &params).unwrap();
    //         target.draw(&grid[&(i as u32)].vertices_buffer, &grid[&(i as u32)].border_indices_buffer, &grid_program, &uniforms, &params).unwrap();
    //     }
    //
    //     if let Some((ref picking_texture, ref depth_buffer)) = picking_attachments {
    //         //clearing the picking texture
    //         picking_texture.main_level().first_layer().into_image(None).unwrap().raw_clear_buffer([0u32, 0, 0, 0]);
    //
    //         let mut picking_target = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&display, picking_texture, depth_buffer).unwrap();
    //         picking_target.clear_depth(1.0);
    //
    //         for i in 1..grid.len()+1 {
    //             let uniforms_texture = uniform! {
    //                 tex: &grass_texture,
    //                 view: view,
    //                 model: model,
    //                 perspective: perspective,
    //             };
    //             picking_target.draw(&grid[&(i as u32)].vertices_buffer, &grid[&(i as u32)].indices_buffer, &picking_program, &uniforms_texture, &params).unwrap();
    //         }
    //     }
    //
    //     if let Some(index) = picked_object {
    //
    //         let center = grid[&index].center;
    //         let radius = 0.4;
    //         let cursor_vertices = [
    //             grid::Vertex { id: 10000, position: (center.0, 0.0, center.1), tex_coords: (0.0, 0.0) },
    //             grid::Vertex { id: 10000, position: (center.0+(0.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(0.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
    //             grid::Vertex { id: 10000, position: (center.0+(1.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(1.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
    //             grid::Vertex { id: 10000, position: (center.0+(2.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(2.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
    //             grid::Vertex { id: 10000, position: (center.0+(3.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(3.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
    //             grid::Vertex { id: 10000, position: (center.0+(4.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(4.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
    //             grid::Vertex { id: 10000, position: (center.0+(5.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(5.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
    //             grid::Vertex { id: 10000, position: (center.0+(6.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(6.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
    //             grid::Vertex { id: 10000, position: (center.0+(7.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(7.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
    //             grid::Vertex { id: 10000, position: (center.0+(8.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(8.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
    //             grid::Vertex { id: 10000, position: (center.0+(9.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(9.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
    //             grid::Vertex { id: 10000, position: (center.0+(10.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(10.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
    //             grid::Vertex { id: 10000, position: (center.0+(11.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(11.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
    //         ];
    //         let cursor_indices: [u32;36] = [0,1,2,0,2,3,0,3,4,0,4,5,0,5,6,0,6,7,0,7,8,0,8,9,0,9,10,0,10,11,0,11,12,0,12,1];
    //
    //         let cursor_vertices_buffer = glium::VertexBuffer::new(&display, &cursor_vertices).unwrap();
    //         let cursor_indices_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &cursor_indices).unwrap();
    //
    //         target.draw(&cursor_vertices_buffer, &cursor_indices_buffer, &select_program, &uniforms, &params).unwrap();
    //         // println!("{:?}", center);
    //         // println!("{:?}", grid[index].coordinates);
    //     }
    //
    //     target.finish().unwrap();
    //
    //     if let (Some(cursor), Some(&(ref picking_texture, _))) = (cursor_position, picking_attachments.as_ref()) {
    //         // println!("{:?}", cursor);
    //         let read_target = glium::Rect {
    //             left: (cursor.0 - 1) as u32,
    //             bottom: (picking_texture.get_height().unwrap() as i32 - std::cmp::max(cursor.1 - 1, 0)) as u32,
    //             width: 1,
    //             height: 1,
    //         };
    //
    //         if read_target.left < picking_texture.get_width()
    //         && read_target.bottom < picking_texture.get_height().unwrap() {
    //             picking_texture.main_level()
    //                 .first_layer()
    //                 .into_image(None).unwrap()
    //                 .raw_read_to_pixel_buffer(&read_target, &picking_pbo);
    //         } else {
    //             picking_pbo.write(&[0]);
    //         }
    //     } else {
    //         picking_pbo.write(&[0]);
    //     }
    //
    //     event_loop.poll_events(|e| {
    //         match e {
    //             glium::glutin::Event::WindowEvent { event, .. } => {
    //                 match event {
    //                     glium::glutin::WindowEvent::CloseRequested => closed = true,
    //                     glium::glutin::WindowEvent::CursorMoved { position, .. } => {
    //                         cursor_position = Some(position.into());
    //                     },
    //                     _ => (),
    //                 }
    //             },
    //             _ => (),
    //         }
    //     });
    //
    //     let now = Instant::now();
    //     let _fps = 1000000000/(now - previous_clock).as_nanos();
    //     println!("{}", _fps);
    //     previous_clock = now;
    // }
}
