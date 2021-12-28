# Rust core for native Android and iOS apps

![Rust](https://github.com/i-schuetz/rust_android_ios/workflows/Rust/badge.svg)
![Android](https://github.com/i-schuetz/rust_android_ios/workflows/Android/badge.svg)
[[TODO](https://github.com/i-schuetz/rust_android_ios/issues/2) iOS badge]

This is an example that shows how to use a shared Rust core in native Android and iOS apps.

⚠️ Looking for maintainers. If you want to help, just open an issue, or email me ivanhp978@gmail.com

# Why?

This approach gives us the best of all worlds: we prevent code duplication by using a shared library. Rust, as a highly performant and safe language is a great fit for mobile. We keep a fully native UI experience and uncomplicated access to the latest APIs of the platforms.

It's also very flexible, allowing to migrate easily between different platforms, including conventional cross-platform frameworks like Flutter or React Native. For example, you can develop your MVP with Rust+React Native or Rust+Flutter, and migrate later to native iOS/Android, without having to rewrite everything. You even can reuse your core for a web-app, using WebAssembly, or desktop app (where again, you can use native or a cross-platform framework like Electron).

# Project structure

- Rust: Repo's root.
- iOS app: `ios_app` directory.
- Android app: Repo's root as well. TODO move it to a folder `android_app`, like the iOS app.

# Possible setups

There are different ways to integrate Rust:

## As source (like in this repo)

- Simple setup.

- Rust is built as part of the app's build process.

- Not ideal if there are team members unfamiliar with Rust.

## As binary

The Rust binary is distributed as an external dependency.

- Better for teams with different skillsets. Everyone works mainly with their familiar tech stack.

- Better separation of concerns / modularity.

- Considerably more complicated to setup than monorepo (has to be done only once though).

- Rust binaries have to be versioned and released. 

Note: it is possible to overwrite the external dependency with a local copy for local development.

## As "normal" library

Here the external dependency contains the Rust binary and wrapper libraries for Android and iOS respectively (written in Kotlin and Swift), which hide the FFI/JNI, providing a simple and safe interface to the apps. This makes working with this dependency like with regular third parties.

An example for this and the binary approaches can be found [here](https://github.com/Co-Epi/app-backend-rust). The Android build contains a wrapper library, which is imported in the Android app with [Gradle](https://github.com/Co-Epi/app-android/blob/54cffa441d27d18ba33d7719a34dc9b5c9125262/app/build.gradle#L168). The iOS build is distributed directly as a binary (no wrapper), using [Carthage](https://github.com/Co-Epi/app-ios/blob/develop/Cartfile#L2).

# Note on concurrency

While it's possible to use asynchronous code in core, it's recommended to use blocking apis and add concurrency in the apps. This simplifies the FFI/JNI interfaces (see the [CoEpi](https://github.com/Co-Epi/app-backend-rust) example, where the apps add concurrency via RxSwift/RxJava).

# "Real world" examples

### [CoEpi](https://github.com/Co-Epi/app-backend-rust)

A mobile contact tracing app for epidemics, with [Android](https://github.com/Co-Epi/app-android) and [iOS](https://github.com/Co-Epi/app-ios) frontends. 

### [Xi editor](https://github.com/xi-editor/xi-editor)

A text editor with a lot of frontends: MacOS, GTK, Electron and Windows, among others.

# Other related projects

### [WASM-Rust-d3 example](https://github.com/i-schuetz/wasm-rust-d3)

An example that shows how to display chart data with d3/JS, using a Rust core to fetch it.

### [yew-d3-example](https://github.com/i-schuetz/yew-d3-example)

Similar to the above, but using the [Yew framework](https://github.com/yewstack/yew)

# Quickstart

Install [rustup](https://rustup.rs/)

## Android specific steps

- Ensure the [NDK](https://developer.android.com/ndk/guides) is installed.

- Set the NDK_HOME environment variable with path to the NDK, e.g:

```
export $NDK_HOME=$HOME/Library/Android/sdk/ndk/21.3.6528147/
```

- Install [cargo ndk](https://github.com/bbqsrc/cargo-ndk):

```
cargo install cargo-ndk
```

- Add targets

```
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
```

- Run the project in Android Studio. This will build Rust, put the binaries in the correct place and start the app.

## iOS specific steps

- Install rust-bitcode 1.46.0 with macOS binary support

```
wget https://github.com/getditto/rust-bitcode/releases/download/v1.46.0/rust-ios-arm64-1.46.0.zip
unzip rust-ios-arm64-1.46.0.zip
cd rust-ios-arm64-1.46.0
./install.sh
```

As the binaries are not signed, you'll have to convince macOS that it's safe to run them.
One solution is to do the following:

1. `cd rust_android_ios/ios_app`
2. `cargo +ios-arm64-1.46.0 build --target aarch64-apple-ios --release --lib`
3. if it fails because macOS doesn't trust the binary, go to 
   `System Preferences -> Security & Privacy` and `Allow to run binary`
    then go to 2.

- Run the project in Xcode. This will build Rust, put the binaries in the correct place and start the app.

## Android specifics

- Logcat doesn't show stdout and stderr, which means that you'll not see `println` or `panic` messages (unless specially configured, like done in this repo).
- If you're having difficulties, try reproducing the problem in a plain (non Android) Kotlin (or Java) project. The JNI is the same, but it's easier to debug, among other things, because you can see stdout/stderr.

## iOS

- iOS shows stdout/stderr as expected and is overall easier to worth with than Android, given the simpler FFI api.

## Inspecting binaries

There are diverse tools to inspect the binaries, e.g.

```
nm -g libcore.so
```

Shows the external symbols, useful if you want to check that the library was generated correctly / contains the symbols from your sources.

To look for a specific symbol:

```
nm -g libcore.so | grep greet
```

# Convenience

## iOS

- [cbindgen](https://github.com/eqrion/cbindgen): generates headers for the FFI Rust declarations. In this project, this would mean that `mobileapp-ios.h` would be automatically generated.

## Android

- [rust-swig](https://github.com/Dushistov/flapigen-rs): similarly to cbindgen for iOS, this generates the JNI api for the Rust declarations. In this project, this would mean that `JNIApi` would be generated, and `ffi_android.rs` mostly too.

# Links

[Official Rust FFI docs](https://doc.rust-lang.org/nomicon/ffi.html)

[Rust FFI guide](https://michael-f-bryan.github.io/rust-ffi-guide/)

[Official JNI docs](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/spec/jniTOC.html) (tutorials may be better to start...)

[Android JNI tips](https://developer.android.com/training/articles/perf-jni)

[Android supported ABIs](https://developer.android.com/ndk/guides/abis)

# Contribute

1. Fork
2. Commit changes to a branch in your fork
3. Push your code and make a pull request

