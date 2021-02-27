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
};

pub struct QueueFamilyIndices {
    pub graphics: Option<usize>,
    pub present: Option<usize>,
}

impl QueueFamilyIndices {
    pub unsafe fn find(app: &Application, pdevice: &vk::PhysicalDevice) -> Self {
        let instance = app.instance.as_ref().unwrap();

        let mut indices = Self {
            graphics: None,
            present: None,
        };

        let queue_family_properties = instance.get_physical_device_queue_family_properties(*pdevice);

        for (i, queue_family) in queue_family_properties.into_iter().enumerate() {
            if queue_family.queue_flags.intersects(vk::QueueFlags::GRAPHICS) {
                indices.graphics = Some(i);
            }

            if indices.is_complete() {
                break;
            }
        }
        indices.present = indices.graphics;

        indices
    }

    pub fn is_complete(&self) -> bool {
        self.graphics.is_some() && self.present.is_some()
    }

    pub fn is_same(&self) -> bool {
        self.graphics == self.present
    }
}

struct Queue {

}
