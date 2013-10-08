extern mod glfw;
extern mod gl;

use gl::types::*;
use glutil::*;

mod glutil;
mod vr;

fn main () -> () {
    do glfw::set_error_callback |_, desc| {
        print(fmt!("GLFW error: %s", desc));
    }

    do glfw::start {
        vr::init();
        let info = vr::get_info();
        println!("Rift resolution: {}x{}", info.HResolution, info.VResolution);
        let window = glfw::Window::create(
            (info.HResolution as uint), (info.VResolution as uint),
            "Holy shit this works", glfw::Windowed).unwrap();
        window.make_context_current();

        // Load gl function pointers.
        gl::load_with(glfw::get_proc_address);
        // Break this frame for profiling purposes.
        window.swap_buffers();

        // Initialization ======================================================

        glfw::set_swap_interval(1);  // VSYNC
        let v_shader = Shader::new("src/perspective.v.glsl", Vertex);
        let f_shader = Shader::new("src/material.f.glsl", Fragment);

        let shaders = [v_shader, f_shader];

        let program = Program::new(shaders);
        program.enable();

        let vertex_data: ~[GLfloat] = ~[
            0.3, 0.1, 0.0,
            0.4, -0.1, 0.0,
            0.3, -0.1, 0.0,

            0.0, 0.0, 0.0,
            -0.3, 0.0, 0.0,
            0.0, -0.3, 0.0,
            ];

        let indices: ~[GLushort] = ~[0, 1, 2, 3, 4, 5];
        let meshes = ~[Mesh::new(6, &vertex_data, &indices)];

        program.enable();

        CheckGLError();


        while !window.should_close() {
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            render_meshes(&program, &meshes);
            window.swap_buffers();
            glfw::poll_events();
        }
        vr::finish();
    }
}
