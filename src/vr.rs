extern mod gl;

pub struct RiftInfo {
    HResolution: u32,
    VResolution: u32,
    HScreenSize: f32,
    VScreenSize: f32,
    VScreenCenter: f32,
    EyeToScreenDistance: f32,
    LensSeparationDistance: f32,
    InterpupillaryDistance: f32,
    DistortionK: [f32, ..4],
    ChromaAbCorrection: [f32, ..4],
    DesktopX: int,
    DesktopY: int
}

#[link_args = "-LOculusSDK/LibOVR/Lib/Linux/Release/x86_64 -lvr -lstdc++ -lovr -ludev -lpthread -lX11 -lXinerama"]
extern {
    fn vr_init();
    fn vr_get_info() -> RiftInfo;
    fn vr_finish();
}

#[fixed_stack_segment]
#[inline(never)]
pub fn init() {
    unsafe {
        vr_init();
    }
}

#[fixed_stack_segment]
#[inline(never)]
pub fn get_info() -> RiftInfo {
    unsafe {
        let info = vr_get_info();
        info
    }
}

#[fixed_stack_segment]
#[inline(never)]
pub fn finish() {
    unsafe {
        vr_finish();
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Non-FFI code:
////////////////////////////////////////////////////////////////////////////////

pub struct Head {
    w: i32,
    h: i32,
    rift_info: RiftInfo,
}

pub fn render_frame(head : &Head, render_func: &fn()) {
    gl::Viewport(0, 0, head.w / 2, head.h);
    render_func();
    gl::Viewport(head.w / 2, 0, head.w / 2, head.h);
    render_func();
}
