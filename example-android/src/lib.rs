use rust_native_core_android::{AndroidApp, App};

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    App::run(app);
}
