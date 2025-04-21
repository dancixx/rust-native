use rust_native_core_android::{AndroidApp, App, JClass, JNIEnv, JObject};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_com_rustnative_MainActivity_nativeInit__Landroid_app_Activity_2(
    env: JNIEnv,
    _class: JClass,
    activity: JObject,
) {
    App::main(&env, activity);
}

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    App::run(app);
}
