use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct ElementId(pub u32);

pub type Callback = Arc<dyn Fn() + Send + Sync + 'static>;

pub trait PlatformRenderer {
    fn create_text(&mut self, text: &str) -> ElementId;
    fn create_button(&mut self, label: &str, on_click: Callback) -> ElementId;
    fn create_container(&mut self) -> ElementId;
    fn add_child(&mut self, parent: ElementId, child: ElementId);
    fn commit(&mut self);
}

pub trait Component {
    fn render(&self, renderer: &mut dyn PlatformRenderer) -> ElementId;
}
