use jni::{
    JNIEnv,
    objects::{JObject, JString},
};

pub fn show_classes<'a>(env: &'a mut JNIEnv<'a>, class_loader: &JObject<'a>) {
    let desc = env
        .call_method(&class_loader, "toString", "()Ljava/lang/String;", &[])
        .unwrap()
        .l()
        .unwrap();

    let str = JString::from(desc);
    let rust_str = env.get_string(&str).unwrap();
    log::info!("ClassLoader: {:?}", rust_str.to_str());
}
