use ahash::AHashMap;
use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use rust_native_core::{Callback, ElementId, PlatformRenderer};

pub struct AndroidRenderer<'a> {
    pub env: &'a mut JNIEnv<'a>,
    pub context: JObject<'a>,
    pub next_id: u32,
    pub views: AHashMap<ElementId, JObject<'a>>,
}

impl<'a> AndroidRenderer<'a> {
    pub fn new(env: &'a mut JNIEnv<'a>, context: JObject<'a>) -> Self {
        AndroidRenderer {
            env,
            context,
            next_id: 0,
            views: AHashMap::new(),
        }
    }

    fn gen_id(&mut self) -> ElementId {
        let id = self.next_id;
        self.next_id += 1;
        ElementId(id)
    }
}

impl<'a> PlatformRenderer for AndroidRenderer<'a> {
    fn create_text(&mut self, text: &str) -> ElementId {
        let id = self.gen_id();
        let text_view = self
            .env
            .new_object(
                "android/widget/TextView",
                "(Landroid/content/Context;)V",
                &[JValue::from(&self.context)],
            )
            .expect("Failed to create TextView");

        let java_str = self.env.new_string(text).unwrap();
        self.env
            .call_method(&text_view, "setText", "(Ljava/lang/CharSequence;)V", &[
                JValue::from(&java_str),
            ])
            .expect("Failed to call setText");
        self.views.insert(id, text_view);
        id
    }

    fn create_button(&mut self, label: &str, on_click: Callback) -> ElementId {
        unimplemented!()
    }

    fn create_container(&mut self) -> ElementId {
        let id = self.gen_id();
        let layout = self
            .env
            .new_object(
                "android/widget/LinearLayout",
                "(Landroid/content/Context;)V",
                &[JValue::from(&self.context)],
            )
            .expect("Failed to create LinearLayout");

        self.env
            .call_method(&layout, "setBackgroundColor", "(I)V", &[JValue::Int(
                0xFFFFFFFFu32 as i32,
            )])
            .expect("Failed to set background color");

        self.views.insert(id, layout);
        id
    }

    fn add_child(&mut self, parent: ElementId, child: ElementId) {
        let parent_view = self.views.get(&parent).expect("Parent view not found");
        let child_view = self.views.get(&child).expect("Child view not found");

        self.env
            .call_method(parent_view, "addView", "(Landroid/view/View;)V", &[
                JValue::from(child_view),
            ])
            .expect("Failed to call addView");
    }

    fn commit(&mut self) {
        let root_view = self.views.get(&ElementId(0)).expect("No root view set");

        self.env
            .call_static_method(
                "com/rustnative/UiHelper",
                "setContentViewOnUiThread",
                "(Landroid/app/Activity;Landroid/view/View;)V",
                &[JValue::from(&self.context), JValue::from(root_view)],
            )
            .expect("Failed to call setContentView via UiHelper");
    }
}
