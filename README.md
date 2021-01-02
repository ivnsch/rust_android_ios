# Rust core for native Android and iOS apps

![logos](img/logos_.png)

![Rust](https://github.com/i-schuetz/rust_android_ios/workflows/Rust/badge.svg)
![Android](https://github.com/i-schuetz/rust_android_ios/workflows/Android/badge.svg)
[TODO iOS badge]

This is an example that shows how to use a shared Rust core in native Android and iOS apps.

# Why?

This approach gives us the best of all worlds: we prevent code duplication by using a shared library. Rust, as a highly performant and safe language is a great fit for mobile. We keep a fully native UI experience and uncomplicated access to the latest APIs of the platforms.

It's also very flexible, allowing to migrate easily between different platforms, including conventional cross-platform frameworks like Flutter or React Native. For example, you can develop your MVP with Rust+React Native or Rust+Fluter, and migrate later to native iOS/Android, without having to rewrite everything. You even can reuse your core for a web-app, using WebAssembly, or desktop app (where again, you can use native or a cross-platform framework like Electron).

# What do I put in Rust?

Everything that's not platform dependent: domain logic, networking, database...

# How do I build modern apps with this?

You probably are wondering how to use Rust with reactive capabilities (RxJava, Combine, reactive database, etc). The answer is that you don't have to manage rx/async in Rust at all (unless e.g. parallelizing computation intensive tasks). The idea that you've to spawn a thread for or put each networking call or database access in an observable, littering your core business logic and services with async flows, is pretty much an anti-pattern (see e.g. this talk https://www.youtube.com/watch?v=BsavoQWAVqM). If you move rx/async near to the UI (where it's needed, to not block the UI thread), the core becomes simpler and easily composable, and you don't have to worry about reactive frameworks in Rust. See [real world example](https://github.com/i-schuetz/rust_android_ios#coepi) below, which implements this pattern: The apps use RxJava and RxSwift, but the core is mostly synchronous (except one place where we need to parallelize a computation intensive task).

# Is this a good fit for my app?

If your app is a thin frontend for a REST api (i.e. the "core" is intended to be simple networking calls), or otherwise UI/platform services -centric, probably it's not worth it. The build flows and maintaining the FFI/JNI interfaces obviously add some complexity to the development process and a new required skillset. If you're in a big company that has plenty of iOS and Android developers, who aren't interested in Rust and don't mind implementing everything 2x, it's probably also not worth it 🙂

If you're a Rust dev with no experience in mobile and don't want to spend time learning 2 platforms, you may prefer a cross-platform UI, like Flutter (see e.g. [this repo](https://github.com/shekohex/flutterust) for how to integrate it with Rust). I personally recommend using native, though, if you want to provide the best user experience, have better tooling, don't want to worry about the cross-platform framework being discontinued, etc.

For everything else I'd say it's at least worth trying out!

# Project structure

- Rust: Repo's root.
- iOS app: `ios_app` directory.
- Android app: Repo's root as well. TODO move it to a folder `android_app`, like the iOS app.

You can open root with IDEs like VSCode or Android Studio. Both have good Rust plugins. Android Studio is probably more convenient to work with Android. To work with the iOS app, you need an IDE that supports it, like Xcode.

# Possible architectures

There are different ways to structure this kind of projects, each with their benefits and drawbacks:

### Monorepo (this repo)

🟢 Simple to configure

🟢 No need to worry about release management for core. It's just regular source code.

🟡 Git history contains Rust/Android/iOS. This can make e.g. release protocols unwieldy. Could be fixed with tooling.

🔴 Not ideal for teams with different skillsets, as everyone has to download everything and it's tightly coupled. If Rust compilation or a related integration step fails, the Android/iOS-only devs can't start the app and are unlikely to know how to fix it.

### Separate repos

Separate repos for Rust/Android/iOS. The Rust binary is distributed as an external, regular dependency for both Android and iOS and it's possible to overwrite it with local builds. And example of this architecture can be found [here](https://github.com/Co-Epi/app-backend-rust)

🟢 Good for teams with different skillsets. iOS and Android devs work with regular apps and never see anything Rust related. Rust developers are not entirely shielded from mobile, as they have to care about FFI/JNI bindings and toolchains, but are mostly also focused.

🟢 Better separation of concerns / modularity. Can lead to better testing, in particular in core (more when using the [separate repos + core wrapper libraries](https://github.com/i-schuetz/rust_android_ios#separate-repos--core-wrapper-libraries) approach).

🟡 Per repo Git history. Not good for release history of apps though, as Rust commits are not included. Could be fixed with tooling.

🟡 Less simple to configure. Though this has to be done only once.

🔴 Rust binaries have to be versioned and released, which can be a bit tedious with frequent changes. Can be improved with good organization though, as it's possible to work locally without releases and the rest of the team doesn't always need every change immediately.

### Separate repos + core wrapper libraries

This is a hybrid between monorepo and separate repos: The Rust repo contains thin wrapper Android and iOS libraries, which perform the FFI/JNI mappings (this is particularly useful for Android, as JNI is far more laborious, and as such, error prone) and related testing, providing a safe interface to the apps.

### Others

Git submodules, etc.

# "Real world" examples

### [CoEpi](https://github.com/Co-Epi/app-backend-rust)

A mobile contact tracing app for epidemics, with [Android](https://github.com/Co-Epi/app-android) and [iOS](https://github.com/Co-Epi/app-ios) frontends. This was developed mostly by the author of this repo. It follows the [separate repos + core wrapper libraries](https://github.com/i-schuetz/rust_android_ios#separate-repos--core-wrapper-libraries) architecture for Android and the [separate repos](https://github.com/i-schuetz/rust_android_ios#separate-repos) architecture for iOS. More explanations can be found in the respective wikis.

### [Xi editor](https://github.com/xi-editor/xi-editor)

A text editor with a lot of frontends: MacOS, GTK, Electron and Windows, among others.

# Other related projects

### [WASM-Rust-d3 example](https://github.com/i-schuetz/wasm-rust-d3)

An example that shows how to display chart data with d3/JS, using a Rust core to fetch it.

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

- Add targets

```
rustup target add x86_64-apple-ios aarch64-apple-ios
```

- Run the project in Xcode. This will build Rust, put the binaries in the correct place and start the app.

# Debugging

So far I've used only logging to debug and it seems fine. I've not missed an automatic debugger. I don't know if it's configurable.

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

- [cbindgen](https://github.com/eqrion/cbindgen): generates headers for the FFI Rust declarations. In this project, this would mean that `mobileapp-ios.h` would be automatically generated. This can be easily integrated in the build process. Writing the headers isn't very tedious or difficult, though, so adding a third party may not be worth it.

## Android

- [rust-swig](https://github.com/Dushistov/flapigen-rs): similarly to cbindgen for iOS, this generates the JNI api for the Rust declarations. In this project, this would mean that `JNIApi` would be generated, and `ffi_android.rs` mostly too. You'd have to write no JNI at all. I _personally_ recommend against this, at least for the beginning, as it's better to understand what's going on and JNI, while tedious, is not so complicated. The files generated by rust-swig are in any case not something I want to debug if something goes wrong.

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

##### If you have any questions or suggestions, open an [issue](https://github.com/i-schuetz/rust_android_ios/issues)!
