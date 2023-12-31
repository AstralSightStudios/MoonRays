use std::ffi::CString;
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{WindowBuilder, Window},
};

pub fn CreateWinitWindow() -> (Window, EventLoop<()>){
    let WindowNameCString = CString::new(crate::GAME_NAME).unwrap();
    let WindowName = WindowNameCString.as_ptr();

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::<u32>::from((800, 600)))
            .with_title(crate::GAME_NAME)
            .build(&event_loop).unwrap();

    return (window, event_loop);
}