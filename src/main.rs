extern mod glfw;
extern mod gl;

use std::cast;
use std::ptr;
use std::sys;

use gl::types::*;
use glutil::*;

mod glutil;

fn render () {
}

static VERTEX_DATA: [GLfloat, ..18] = [
    0.3, 0.1, 0.0,
    0.4, -0.1, 0.0,
    0.3, -0.1, 0.0,
    0.0, 0.0, 0.0,
    -0.1, 0.0, 0.0,
    0.0, -0.1, 0.0,
];

fn create_triangles() -> (GLuint, GLuint) {
    unsafe {
        let mut vbo: GLuint = 0;
        let mut vib: GLuint = 0;

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * sys::size_of::<GLfloat>()) as GLsizeiptr,
            cast::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW);
        CheckGLError();

        let indices: [GLushort, ..6] = [0, 1, 2, 3, 4, 5];

        gl::GenBuffers(1, &mut vib);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vib);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * sys::size_of::<GLushort>()) as GLsizeiptr,
            cast::transmute(&indices[0]),
            gl::STATIC_DRAW);
        CheckGLError();
        (vbo, vib)
    }
}
fn draw_triangles(program: &Program, vbo: GLuint, vib: GLuint) {
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let pos_attr = "position".with_c_str(|ptr| gl::GetAttribLocation(program.gl_id, ptr));
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vib);
        gl::DrawElements(
            gl::TRIANGLES, 18, gl::UNSIGNED_SHORT, ptr::null());
        CheckGLError();
    }
}

fn main () -> () {
    do glfw::set_error_callback |_, desc| {
        print(fmt!("GLFW error: %s", desc));
    }

    do glfw::start {
        //glfw::window_hint::context_version(3, 2);
        //glfw::window_hint::opengl_profile(glfw::OpenGlCoreProfile);
        let window = glfw::Window::create(640, 480, "Holy shit this works", glfw::Windowed).unwrap();
        window.make_context_current();

        // Load gl function pointers.
        gl::load_with(glfw::get_proc_address);
        // Break this frame for profiling purposes.
        window.swap_buffers();

        // Initialization ======================================================
        let v_shader = Shader::new("src/perspective.v.glsl", Vertex);
        let f_shader = Shader::new("src/material.f.glsl", Fragment);

        let shaders = [v_shader, f_shader];

        let program = Program::new(shaders);
        program.enable();


        let (vbo, vib) = create_triangles();

        program.enable();

        CheckGLError();

        while !window.should_close() {
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            draw_triangles(&program, vbo, vib);
            window.swap_buffers();
            glfw::poll_events();
        }
    }
}
