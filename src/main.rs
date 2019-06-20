#![macro_use]
extern crate glium;
extern crate image;

// use glium::{glutin, Surface};
use std::io::Cursor;
use glium::{uniform, Surface};

mod grid;

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

    let mut closed = false;

    let image = image::load(Cursor::new(&include_bytes!("../ressources/grass.png")[..]), image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let grass_texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("../ressources/dirt.png")[..]), image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let dirt_texture = glium::texture::Texture2d::new(&display, image).unwrap();
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
    uniform mat4 view;
    uniform mat4 perspective;

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

    let perspective: [[f32; 4]; 4] = {
        let right: f32 = 10.0;
        let left: f32 = -10.0;
        let bottom: f32 = -10.0;
        let top: f32 = 10.0;
        let zfar: f32 = 10.0;
        let znear: f32 = -10.0;

        [
            [2.0/(right-left)   ,    0.0,              0.0              ,   -(right+left)/(right-left)],
            [         0.0         ,     2.0/(top-bottom) ,              0.0              ,   -(top+bottom)/(top-bottom)],
            [         0.0         ,    0.0,  -2./(zfar-znear)    ,   -(zfar+znear)/(zfar-znear)],
            [         0.0         ,    0.0, 0.0,   1.0f32],
        ]
    };

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let grid_program = glium::Program::from_source(&display, vertex_shader_grid_src, fragment_shader_grid_src, None).unwrap();

    let mut grid: Vec<grid::Hex> = Vec::new();
    let n = 5;
    for x in -n..n {
        for y in -n..n {
            for z in -n..n {
                grid.push(grid::Hex::new(&display, x,y,z));
            }
        }
    }

    let view = view_matrix(&[0.0, 0.0, 0.0], &[0.0, 0.0, -1.0], &[0.0, 1.0, 0.0]);

    let uniforms = uniform! {
        tex: &grass_texture,
        view: view,
        perspective: perspective,
        model: [
        [0.70710678118651090, -0.40824829065509560, -0.57735026905444890, 0.0],
[0.00, 0.81649658073651470, -0.57735026946003950, 0.0],
[0.70710678118658420, 0.40824829065505330, 0.5773502690543890, 0.0],
        [0.0, 00.0, 0.0, 1.0f32]
        ]
    };

    while !closed {
        let mut target = display.draw();
        target.clear_color(0.9, 0.9, 0.9, 1.0);

        for i in 0..grid.len() {
            target.draw(&grid[i].vertices_buffer, &grid[i].border_indices_buffer, &grid_program, &uniforms, &Default::default()).unwrap();
            target.draw(&grid[i].vertices_buffer, &grid[i].indices_buffer, &program, &uniforms, &Default::default()).unwrap();
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
