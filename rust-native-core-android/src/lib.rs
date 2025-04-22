use ahash::AHashMap;
use android_activity::{MainEvent, PollEvent};
use anyhow::Result;
use jni::JavaVM;
use jni::objects::{GlobalRef, JValue};
use rust_native_core::{Callback, Component, ElementId, PlatformRenderer};
use rust_native_ui::android::text::Text;

pub mod utils;
pub use android_activity::AndroidApp;
pub use jni::JNIEnv;
pub use jni::objects::{JClass, JObject};

pub struct App;

impl App {
    pub fn main<'a>(env: &'a JNIEnv<'a>, activity: JObject<'a>) {
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "1");
        }

        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Trace)
                .with_tag("RUST_NATIVE"),
        );

        log::info!("App started");

        let vm = env.get_java_vm().unwrap();
        let mut env = Box::leak(Box::new(vm.attach_current_thread().unwrap()));
        let mut renderer = AndroidRenderer::new(&mut env, activity).unwrap();

        let container = renderer.create_container();
        let texts = vec!["Hello", "from", "Rust-native"];
        for text in texts {
            let view = Text::new(text);
            let id = view.render(&mut renderer);
            renderer.add_child(container, id);
        }
        renderer.commit();
    }

    pub fn run(app: AndroidApp) {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Trace)
                .with_tag("RUST_NATIVE"),
        );

        loop {
            app.poll_events(Some(std::time::Duration::from_millis(500)), |event| {
                match event {
                    PollEvent::Main(MainEvent::Destroy) => {
                        log::info!("Activity destroyed. Exiting.");
                        std::process::exit(0);
                    }
                    PollEvent::Main(main_event) => {
                        log::info!("Main event: {:?}", main_event);
                    }
                    PollEvent::Timeout => {
                        log::info!("Polling timeout tick");
                    }
                    PollEvent::Wake => {
                        log::info!("Wake event");
                    }
                    _ => {}
                }

                // app.input_events(|input_event| {
                //     log::info!("Input Event: {:?}", input_event);
                //     InputStatus::Unhandled
                // });
            });
        }
    }
}

pub struct AndroidRenderer<'a> {
    pub env: &'a mut JNIEnv<'a>,
    pub activity: JObject<'a>,
    pub class_loader: GlobalRef,
    pub next_id: u32,
    pub views: AHashMap<ElementId, JObject<'a>>,
}

impl<'a> AndroidRenderer<'a> {
    pub fn new(env: &'a mut JNIEnv<'a>, activity: JObject<'a>) -> Result<Self> {
        let class_loader = env
            .call_method(
                &activity,
                "getClassLoader",
                "()Ljava/lang/ClassLoader;",
                &[],
            )?
            .l()?;
        let class_loader = env.new_global_ref(&class_loader)?;

        Ok(AndroidRenderer {
            env,
            activity,
            class_loader,
            next_id: 0,
            views: AHashMap::new(),
        })
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
                &[JValue::from(&self.activity)],
            )
            .expect("Failed to create TextView");

        let java_str = self.env.new_string(text).unwrap();
        self.env
            .call_method(
                &text_view,
                "setText",
                "(Ljava/lang/CharSequence;)V",
                &[JValue::from(&java_str)],
            )
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
                &[JValue::from(&self.activity)],
            )
            .expect("Failed to create LinearLayout");

        self.env
            .call_method(
                &layout,
                "setBackgroundColor",
                "(I)V",
                &[JValue::Int(0xFFFFFFFFu32 as i32)],
            )
            .expect("Failed to set background color");

        self.views.insert(id, layout);
        id
    }

    fn add_child(&mut self, parent: ElementId, child: ElementId) {
        let parent_view = self.views.get(&parent).expect("Parent view not found");
        let child_view = self.views.get(&child).expect("Child view not found");

        self.env
            .call_method(
                parent_view,
                "addView",
                "(Landroid/view/View;)V",
                &[JValue::from(child_view)],
            )
            .expect("Failed to call addView");
    }

    fn commit(&mut self) {
        let root_view = self.views.get(&ElementId(0)).expect("No root view set");

        let class_name = self.env.new_string("com.rustnative.UiHelper").unwrap();
        let uihelper_class = self
            .env
            .call_method(
                &self.class_loader,
                "loadClass",
                "(Ljava/lang/String;)Ljava/lang/Class;",
                &[JValue::Object(&class_name)],
            )
            .unwrap()
            .l()
            .unwrap();

        let result = self.env.call_static_method(
            &JClass::from(uihelper_class),
            "setContentViewOnUiThread",
            "(Landroid/app/Activity;Landroid/view/View;)V",
            &[JValue::Object(&self.activity), JValue::Object(root_view)],
        );

        match result {
            Ok(_) => log::info!("Successfully called setContentViewOnUiThread"),
            Err(e) => {
                log::error!("JavaException: {:?}", e);

                // Log Java stacktrace to logcat
                if let Err(inner) = self.env.exception_describe() {
                    log::error!("Failed to describe exception: {:?}", inner);
                }

                // Clear the Java exception so it doesn't crash the VM
                if let Err(inner) = self.env.exception_clear() {
                    log::error!("Failed to clear exception: {:?}", inner);
                }

                panic!("JavaException occurred: {:?}", e);
            }
        }
    }
}
