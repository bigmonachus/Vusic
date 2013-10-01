extern mod glfw;
extern mod gl;

use gl::types::*;

fn render () {
    gl::ClearColor(1.0, 1.0, 1.0, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
}

fn main () -> () {
    do glfw::set_error_callback |_, desc| {
        print(fmt!("GLFW error: %s", desc));
    }
    println(fmt!("Hello world, glViewport is: %?", gl::Viewport));
    do glfw::start {
        let window = glfw::Window::create(640, 480, "Holy shit this works", glfw::Windowed).unwrap();
        window.make_context_current();

        // Load gl function pointers.
        gl::load_with(glfw::get_proc_address);

        while !window.should_close() {
            render();
            window.swap_buffers();
            glfw::poll_events();
        }
    }
}
