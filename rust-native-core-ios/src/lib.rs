use std::{
    collections::HashMap,
    ffi::{CString, c_char},
};

use rust_native_core::{Callback, ElementId, PlatformRenderer};

unsafe extern "C" {
    fn create_text(text: *const c_char) -> u32;
    fn create_button(label: *const c_char, callback_id: u32) -> u32;
    fn create_container() -> u32;
    fn add_child(parent: u32, child: u32);
    fn commit();
}

pub struct IOSRenderer {
    callbacks: HashMap<u32, Callback>,
    next_id: u32,
}

impl IOSRenderer {
    pub fn new() -> Self {
        IOSRenderer {
            callbacks: HashMap::new(),
            next_id: 0,
        }
    }

    fn register_callback(&mut self, callback: Callback) -> u32 {
        let id = self.next_id;
        self.callbacks.insert(id, callback);
        self.next_id += 1;
        id
    }

    pub fn handle_callback(&mut self, id: u32) {
        if let Some(callback) = self.callbacks.remove(&id) {
            callback();
        }
    }
}
impl PlatformRenderer for IOSRenderer {
    fn create_text(&mut self, text: &str) -> ElementId {
        let c = CString::new(text).unwrap();
        let id = unsafe { create_text(c.as_ptr()) };
        ElementId(id)
    }

    fn create_button(&mut self, label: &str, on_click: Callback) -> ElementId {
        let cb_id = self.register_callback(on_click);
        let c = CString::new(label).unwrap();
        let id = unsafe { create_button(c.as_ptr(), cb_id) };
        ElementId(id)
    }

    fn create_container(&mut self) -> ElementId {
        ElementId(unsafe { create_container() })
    }

    fn add_child(&mut self, parent: ElementId, child: ElementId) {
        unsafe { add_child(parent.0, child.0) }
    }

    fn commit(&mut self) {
        unsafe { commit() }
    }
}
