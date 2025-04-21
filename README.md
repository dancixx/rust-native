# rust-native · Android Integration

**`rust-native`** is a declarative UI framework written in Rust, inspired by React Native. This project demonstrates how to integrate your Rust UI logic into a native Android application using JNI.

---

## 📦 Build the Native Rust Library

Use [`cargo-ndk`](https://github.com/bbqsrc/cargo-ndk) to build the shared library (`.so`) for Android:

```bash
cargo ndk -t arm64-v8a -o example-android/app/src/main/jniLibs build --release
```

This will generate `libmain.so` inside:

```
example-android/app/src/main/jniLibs/arm64-v8a/libmain.so
```

This file is loaded from the Android app via JNI.

---

## 📱 Run on Android Emulator or Device

To build, install, and launch the Android app:

```bash
cd example-android

./gradlew assembleDebug
./gradlew installDebug
adb shell monkey -p com.rustnative -c android.intent.category.LAUNCHER 1
adb logcat | grep -A 20 "RUST_NATIVE"
```

- `monkey` simulates an app launch
- `logcat` filters logs from your native Rust code (`println!` mapped to `__android_log_print`)

---

## 🔎 Verify Java JNI Integration

Make sure the Java glue code (`UiHelper.java`) is included in the built APK:

```bash
unzip -p example-android/app/build/outputs/apk/debug/app-debug.apk classes.dex | strings | grep -i UiHelper
```

This checks for the `UiHelper` class inside the APK.

---

## 📁 Project Structure

```
example-android/
├── src/
│   └── lib.rs                   # Rust native library code
├── Cargo.toml                   # Rust package manifest

├── app/
│   └── src/main/
│       ├── java/com/rustnative/
│       │   └── UiHelper.java    # Java helper to call Rust code
│       ├── jniLibs/arm64-v8a/
│       │   └── libmain.so       # Compiled Rust library
│       └── AndroidManifest.xml  # Android entry point

├── build.gradle                 # Project-level Gradle config
├── settings.gradle
├── gradlew / gradlew.bat
└── gradle/
    └── wrapper/
        ├── gradle-wrapper.jar
        └── gradle-wrapper.properties
```

---
Built with ❤️ in Rust
