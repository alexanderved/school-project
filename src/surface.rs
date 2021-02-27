use winit::{
    event_loop::*,
    window,
    window::*,
    dpi::*,
};
use ash::{
    Entry,
    Instance,
    version::EntryV1_0,
    vk,
    extensions::{
        ext::DebugUtils,
        khr,
    },
};
use std::{
    ffi::CString,
    sync::Arc,
};
use crate::Application;

pub struct Surface {
    pub window: Arc<window::Window>,
    pub surface: vk::SurfaceKHR,
    pub surface_loader: khr::Surface,
}

impl Surface {
    pub unsafe fn new(app: &mut Application) -> Option<Self> {
        let entry = app.entry.as_ref()?;
        let instance = app.instance.as_ref()?;

        let surface = ash_window::create_surface(entry, instance, &*app.window, None).ok()?;
        let surface_loader = khr::Surface::new(entry, instance);

        Some(Self {
            window: Arc::clone(&app.window),
            surface,
            surface_loader,
        })
    }
}
