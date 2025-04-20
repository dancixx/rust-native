fn main() {
    println!("cargo:rerun-if-changed=android/app/src/main/java/com/rustnative/UiHelper.java");
}
