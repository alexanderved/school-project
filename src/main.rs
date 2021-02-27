mod surface;
mod physical_device;
mod queue;

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
        khr::{Surface, Swapchain},
    },
};
use std::{
    ffi::CString,
    sync::Arc,
};

pub struct Application {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Arc<window::Window>,

    pub entry: Option<Entry>,
    pub instance: Option<Instance>,

    pub surface: Option<surface::Surface>,

    pub pdevice: Option<physical_device::PhysicalDevice>,
}

impl Application {
    fn new() -> Application {
        let event_loop = EventLoop::new();
        let window = Arc::new(WindowBuilder::new()
            .with_title("School Project")
            .with_inner_size(LogicalSize::new(800, 600))
            .build(&event_loop)
            .unwrap());

        let mut app = Application {
            event_loop: Some(event_loop),
            window,

            entry: None,
            instance: None,

            surface: None,

            pdevice: None,
        };

        app.init_vulkan();

        app
    }

    fn init_vulkan(&mut self) {
        self.entry = Entry::new().ok();

        unsafe {
            self.create_instance();
            self.surface = surface::Surface::new(self);
            self.pdevice = physical_device::PhysicalDevice::pick(self);
        }
    }

    unsafe fn create_instance(&mut self) {
        let entry = self.entry.as_ref().unwrap();

        let app_name = CString::new("School Project").unwrap();
        let application_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(0)
            .engine_name(&app_name)
            .engine_version(0)
            .api_version(vk::make_version(1, 0, 0));

        let surface_extensions = ash_window::enumerate_required_extensions(&*self.window).unwrap();
        let mut extension_names_raw = surface_extensions
            .iter()
            .map(|ext| ext.as_ptr())
            .collect::<Vec<_>>();
        extension_names_raw.push(DebugUtils::name().as_ptr());

        let layer_names = [CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
        let layers_names_raw: Vec<*const i8> = layer_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        let instance_create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(&extension_names_raw)
            .enabled_layer_names(&layers_names_raw)
            .build();

        self.instance = entry.create_instance(&instance_create_info, None).ok();
    }

    fn run(mut self) -> ! {
        self.event_loop.take().unwrap().run(|_, _, _| {

        });
    }
}

fn main() {
    let app = Application::new();
    app.run();
}
