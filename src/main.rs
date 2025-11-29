use ash::{Entry, Instance, vk};
use glfw::*;
use std::ffi::CString;

fn create_vulkan_instance() -> ash::Instance {
    let entry = unsafe { Entry::load().unwrap() };

    let application_name = CString::new("Vrykul Renderer").unwrap();
    let engine_name = CString::new("Vrykul").unwrap();
    let app_info = vk::ApplicationInfo::default()
        .application_name(&application_name)
        .engine_name(&engine_name)
        .api_version(vk::make_api_version(0, 1, 3, 0));

    let mut enabled_layers: Vec<CString> = Vec::new();
    #[cfg(debug_assertions)]
    enabled_layers.push(CString::new("VK_LAYER_KHRONOS_validation").unwrap());

    let instance_create_info = vk::InstanceCreateInfo::default().application_info(&app_info);
    let instance = unsafe { entry.create_instance(&instance_create_info, None).unwrap() };

    return instance;
}

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    create_vulkan_instance();

    let (window, _) = glfw
        .create_window(1280, 720, "Vrykul Renderer", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    while !window.should_close() {
        glfw.poll_events();
    }
}
