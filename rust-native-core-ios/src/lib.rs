use std::{
    cell::RefCell,
    ffi::{CString, c_char},
    sync::Arc,
};

use ahash::AHashMap;
use objc2::{rc::Retained, runtime::AnyObject};
use rust_native_core::{Callback, ElementId, PlatformRenderer};

thread_local! {
    static IOS_RENDERER: RefCell<Option<IOSRenderer>> = RefCell::new(None);
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_app_main() {
    let mut renderer = IOSRenderer::new();

    let root = renderer.create_container();
    let text = renderer.create_text("Welcome to Rust+iOS");
    let btn = renderer.create_button(
        "Click",
        Arc::new(|| {
            println!("Button clicked");
        }),
    );

    renderer.add_child(root, text);
    renderer.add_child(root, btn);
    renderer.commit();

    IOS_RENDERER.with(|cell| *cell.borrow_mut() = Some(renderer));
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_handle_callback(id: u32) {
    IOS_RENDERER.with(|cell| {
        if let Some(renderer) = cell.borrow_mut().as_mut() {
            renderer.handle_callback(id);
        }
    });
}

unsafe extern "C" {
    fn create_text(text: *const c_char) -> u32;
    fn create_button(label: *const c_char, callback_id: u32) -> u32;
    fn create_container() -> u32;
    fn add_child(parent: u32, child: u32);
    fn commit();
}

pub struct IOSRenderer {
    callbacks: AHashMap<u32, Callback>,
    next_id: u32,
    pub views: AHashMap<ElementId, Retained<AnyObject>>,
}

impl IOSRenderer {
    pub fn new() -> Self {
        Self {
            callbacks: AHashMap::new(),
            next_id: 0,
            views: AHashMap::new(),
        }
    }

    fn register_callback(&mut self, callback: Callback) -> u32 {
        let id = self.next_id;
        self.callbacks.insert(id, callback);
        self.next_id += 1;
        id
    }

    pub fn handle_callback(&mut self, id: u32) {
        if let Some(cb) = self.callbacks.remove(&id) {
            cb();
        }
    }
}

impl PlatformRenderer for IOSRenderer {
    fn create_text(&mut self, text: &str) -> ElementId {
        let c = CString::new(text).expect("CString::new failed");
        let id = unsafe { create_text(c.as_ptr()) };
        ElementId(id)
    }

    fn create_button(&mut self, label: &str, on_click: Callback) -> ElementId {
        let cb_id = self.register_callback(on_click);
        let c = CString::new(label).expect("CString::new failed");
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
