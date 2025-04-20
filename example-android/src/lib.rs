use once_cell::sync::OnceCell;

use android_activity::{AndroidApp, InputStatus, MainEvent, PollEvent};
use jni::{
    JNIEnv, JavaVM, NativeMethod,
    objects::{GlobalRef, JClass, JObject, JValue},
    strings::JNIString,
    sys::JNINativeMethod,
};
use rust_native_core::{Component, PlatformRenderer};
use rust_native_core_android::AndroidRenderer;
use rust_native_ui::text::Text;

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    log::info!("App started");

    let vm = unsafe { JavaVM::from_raw(app.vm_as_ptr() as *mut jni::sys::JavaVM) };
    let vm = vm.unwrap();
    let mut env = Box::leak(Box::new(vm.attach_current_thread().unwrap()));
    let context = unsafe { JObject::from_raw(app.activity_as_ptr() as *mut jni::sys::_jobject) };

    let mut renderer = AndroidRenderer::new(&mut env, context);

    let container = renderer.create_container();
    let texts = vec!["Hello", "from", "Rust-native"];
    for text in texts {
        let view = Text::new(text);
        let id = view.render(&mut renderer);
        renderer.add_child(container, id);
    }

    loop {
        app.poll_events(Some(std::time::Duration::from_millis(500)), |event| {
            match event {
                PollEvent::Main(MainEvent::Destroy) => {
                    log::info!("Activity destroyed. Exiting.");
                    std::process::exit(0);
                }
                PollEvent::Main(main_event) => {
                    log::info!("Main event: {:?}", main_event);
                    renderer.commit();
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
