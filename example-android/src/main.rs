use android_activity::{AndroidApp, InputStatus, MainEvent, PollEvent};
use jni::{JavaVM, objects::JObject};
use rust_native_core::{Component, PlatformRenderer};
use rust_native_core_android::AndroidRenderer;
use rust_native_ui::text::Text;

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
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
    let text = Text::new("Hello from Rust-native!");
    let text_id = text.render(&mut renderer);
    renderer.add_child(container, text_id);
    renderer.commit();

    // Eseménykezelő ciklus
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
