extern mod glfw;

fn main () -> () {
    do glfw::set_error_callback |_, desc| {
        print(fmt!("GLFW error: %s", desc));
    }
    println("Hello world");
    do glfw::start {
        let window = glfw::Window::create(640, 480, "Holy shit this works", glfw::Windowed).unwrap();
        window.make_context_current();
        while !window.should_close() {
            window.swap_buffers();
            glfw::poll_events();
        }
    }
}
