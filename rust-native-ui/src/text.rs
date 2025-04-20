use rust_native_core::{Component, ElementId, PlatformRenderer};

pub struct Text {
    pub content: String,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl Component for Text {
    fn render(&self, renderer: &mut dyn PlatformRenderer) -> ElementId {
        renderer.create_text(&self.content)
    }
}
