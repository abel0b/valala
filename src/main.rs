#![macro_use]
extern crate glium;
extern crate image;

// use glium::{glutin, Surface};
use std::io::Cursor;
use glium::{uniform, Surface};

mod grid;
use grid::Vertex;

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

fn main() {
    let mut event_loop = glium::glutin::EventsLoop::new();

    let wb = glium::glutin::WindowBuilder::new();
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let mut t:f32 = -0.5;

    let mut closed = false;

    // let vertices:[Vertex; 6] = [
    //      Vertex { position: (-0.5, -0.5, 0.0), tex_coords: (0.0, 0.0) },
    //      Vertex { position: (-0.5, 0.5, 0.0), tex_coords: (0.0, 1.0) },
    //      Vertex { position: (0.5, -0.5, 0.0), tex_coords: (0.333333333, 0.0) },
    //      Vertex { position: (0.5, 0.5, 0.0), tex_coords: (0.333333333, 1.0) },
    //      Vertex { position: (-0.5, 0.5, 1.0), tex_coords: (0.333333333, 1.0) },
    //      Vertex { position: (0.5, 0.5, 1.0), tex_coords: (0.333333333, 1.0) },
    // ];
    //
    //
    // let indices:[u16;12] = [
    //     0, 1, 2,
    //     1, 2, 3,
    //     1, 3, 4,
    //     3, 4, 5,
    // ];

    let image = image::load(Cursor::new(&include_bytes!("../ressources/grass.png")[..]), image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let grass_texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("../ressources/dirt.png")[..]), image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let dirt_texture = glium::texture::Texture2d::new(&display, image).unwrap();

    while !closed {
        t = t + 0.0002;
        if t > 0.5 {
            t = -0.5;
        }
        let mut target = display.draw();
        target.clear_color(0.9, 0.9, 0.9, 1.0);

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };

        let vertex_shader_src = r#"
            #version 140

            in vec3 position;
            in vec2 tex_coords;
            uniform mat4 model;
            uniform mat4 perspective;
            uniform mat4 view;

            out vec2 v_tex_coords;

            void main() {
                v_tex_coords = tex_coords;
                mat4 modelview = view * model;
                gl_Position =  perspective * modelview * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            in vec2 v_tex_coords;
            uniform sampler2D tex;

            out vec4 color;

            void main() {
                color = texture(tex, v_tex_coords);
            }
        "#;


        let vertex_shader_grid_src = r#"
            #version 140

            in vec3 position;
            uniform mat4 model;
            uniform mat4 perspective;
            uniform mat4 view;

            void main() {
                mat4 modelview = view * model;
                gl_Position =  perspective * modelview * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_grid_src = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(0.1, 0.1, 0.1, 1.0);
            }
        "#;

        // let view = view_matrix(&[0.70710678118, 0.0, -0.70710678118], &[0.40824829046, 0.81649658092, 0.40824829046], &[0.57735026919, -0.57735026919, 0.57735026919]);
        let view = view_matrix(&[0.0, 0.0, 0.0], &[0.0, 0.0, 0.1], &[0.0, 1.0, 0.0]);

        let uniforms = uniform! {
            tex: &grass_texture,
            perspective: perspective,
            view: view,
            model: [
                [0.70710678, -0.31661349, -0.63226252, 0.0],
                [0.0,          0.89415424, -0.44775909, 0.0],
                [0.70710678, 0.31661349,  0.63226252,  0.0],
                [0.0, 0.0, 10.0, 1.0f32]
            ]
        };

        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
        let grid_program = glium::Program::from_source(&display, vertex_shader_grid_src, fragment_shader_grid_src, None).unwrap();
        for x in -5..5 {
            for y in -5..5 {
                for z in -5..5 {
                    let cell = grid::Hex::new(x,y,z);
                    let vertices: [Vertex; 7] = cell.vertices();
                    let indices: [u32; 18] = cell.indices();
                    let border_indices: [u32;12] = cell.border_indices();

                    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
                    let index_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();
                    let grid_index_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::LinesList, &border_indices).unwrap();

                    target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
                    target.draw(&vertex_buffer, &grid_index_buffer, &grid_program, &uniforms, &Default::default()).unwrap();
                }
            }
        }
        target.finish().unwrap();

        event_loop.poll_events(|e| {
            match e {
                glium::glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glium::glutin::WindowEvent::CloseRequested => closed = true,
                        _ => (),
                    }
                },
                _ => (),
            }
        })
    }
}
