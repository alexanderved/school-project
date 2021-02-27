use winit::{
    event_loop::*,
    window,
    window::*,
    dpi::*,
};
use ash::{
    Entry,
    Instance,
    version::{EntryV1_0, InstanceV1_0},
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
use crate::{
    Application,
    queue::*,
};

pub struct PhysicalDevice {
    pub pdevice: vk::PhysicalDevice,
}

impl PhysicalDevice {
    pub unsafe fn pick(app: &Application) -> Option<Self> {
        let instance = app.instance.as_ref()?;
        let pdevice = instance.enumerate_physical_devices()
            .ok()?
            .into_iter()
            .find(|pdevice| Self::is_device_suitable(app, pdevice))?;

        Some(Self {
            pdevice,
        })
    }

    fn is_device_suitable(app: &Application, pdevice: &vk::PhysicalDevice) -> bool {
        let indices = unsafe { QueueFamilyIndices::find(app, pdevice) };

        indices.is_complete()
    }
}
