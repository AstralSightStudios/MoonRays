use ash::{self, Instance, Entry, vk::SurfaceKHR, extensions::khr::Surface};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use winit::window::Window;

pub fn GetSurface(VkInstance: &Instance, entry: &Entry, window: &Window) -> (SurfaceKHR, Surface){
    unsafe{
        let surface = ash_window::create_surface(
            entry,
            VkInstance,
            window.raw_display_handle(),
            window.raw_window_handle(),
            None,
        ).unwrap();

        let surface_fn = ash::extensions::khr::Surface::new(&entry, &VkInstance);

        return (surface, surface_fn);
    }
}