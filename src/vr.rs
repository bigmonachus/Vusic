#[link_args = "-LOculusSDK/LibOVR/Lib/Linux/Release/x86_64 -lvr -lstdc++ -lovr -ludev -lpthread -lX11 -lXinerama"]
extern {
    fn vr_init();
    fn vr_finish();
}

#[fixed_stack_segment]
#[noinline]
pub fn init() {
    unsafe {
        vr_init();
    }
}

#[fixed_stack_segment]
#[noinline]
pub fn finish() {
    unsafe {
        vr_finish();
    }
}
