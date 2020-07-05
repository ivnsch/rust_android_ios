use std::{
    sync::mpsc::{self, Sender},
    thread,
};
extern crate jni;
use self::jni::JNIEnv;
use jni::objects::{GlobalRef, JClass, JObject, JString, JValue};
use jni::sys::{jint, jobject, jstring};
use jni::JavaVM;
use mpsc::Receiver;

// TODO no_mangle needed?
#[no_mangle]
pub unsafe extern "C" fn Java_com_schuetz_rustandroidios_JniApi_greet(
    env: JNIEnv,
    _: JClass,
    who: JString,
) -> jstring {
    let str: String = env.get_string(who)
        .expect("Couldn't create java string").into();

    let output = env.new_string(format!("Hello 👋 {}!", str))
        .expect("Couldn't create java string");

    output.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_schuetz_rustandroidios_JniApi_add(
    _env: JNIEnv,
    _: JClass,
    value1: jint,
    value2: jint,
) -> jint {
    println!("Passed value1: {}, value2: {}", value1, value2);
    value1 + value2
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_schuetz_rustandroidios_JniApi_passObject(
    env: JNIEnv,
    _: JClass,
    object: JObject,
) {
    let my_int_j_value_res = env.get_field(object, "intPar", "I");
    let my_int: i32 = my_int_j_value_res.unwrap().i().unwrap();

    let my_str_j_value = env.get_field(object, "stringPar", "Ljava/lang/String;")
        .expect("Couldn't get JValue");
    let my_str_j_object = my_str_j_value.l();
    let my_str_j_string = JString::from(my_str_j_object.unwrap());

    let my_str_java_string = env.get_string(my_str_j_string).unwrap();
    let my_str = my_str_java_string.to_str().unwrap();

    println!("Passed: {}, {}", my_int, my_str);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_schuetz_rustandroidios_JniApi_returnObject(
    env: JNIEnv,
    _: JClass,
) -> jobject {
    let cls = env.find_class("com/schuetz/rustandroidios/Dummy");

    let my_int_j_value = JValue::from(123);

    let str_parameter_j_string = env.new_string("my string parameter")
        .expect("Couldn't create java string!");
    let str_parameter_j_value = JValue::from(JObject::from(str_parameter_j_string));

    let obj = env.new_object(
        cls.unwrap(),
        "(Ljava/lang/String;I)V",
        &[str_parameter_j_value, my_int_j_value],
    );

    obj.unwrap().into_inner()
}

pub static mut CALLBACK_SENDER: Option<Sender<String>> = None;

#[no_mangle]
pub unsafe extern "C" fn Java_com_schuetz_rustandroidios_JniApi_registerCallback(
    env: JNIEnv,
    _: JClass,
    callback: jobject,
) -> jint {
    let my_callback = MyCallbackImpl {
        java_vm: env.get_java_vm().unwrap(),
        callback: env.new_global_ref(callback).unwrap(),
    };
    register_callback_internal(Box::new(my_callback));

    // Let's send a message immediately, to test it
    send_to_callback("Hello callback!".to_owned());

    1
}

unsafe fn send_to_callback(string: String) {
    match &CALLBACK_SENDER {
        Some(s) => {
            s.send(string).expect("Couldn't send message to callback!");
        }
        None => {
            println!("No callback registered");
        }
    }
}

fn register_callback_internal(callback: Box<dyn MyCallback>) {
    // Make callback implement Send (marker for thread safe, basically) https://doc.rust-lang.org/std/marker/trait.Send.html
    let my_callback =
        unsafe { std::mem::transmute::<Box<dyn MyCallback>, Box<dyn MyCallback + Send>>(callback) };

    // Create channel
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    // Save the sender in a static variable, which will be used to push elements to the callback
    unsafe {
        CALLBACK_SENDER = Some(tx);
    }

    // Thread waits for elements pushed to SENDER and calls the callback
    thread::spawn(move || {
        for string in rx.iter() {
            my_callback.call(string)
        }
    });
}

trait MyCallback {
    fn call(&self, par: String);
}

struct MyCallbackImpl {
    // The callback passed from Android is a local reference: only valid during the method call.
    // To store it, we need to put it in a global reference.
    // See https://developer.android.com/training/articles/perf-jni#local-and-global-references
    callback: GlobalRef,

    // We need JNIEnv to call the callback.
    // JNIEnv is valid only in the same thread, so we have to store the vm instead, and use it to get
    // a JNIEnv for the current thread.
    // See https://developer.android.com/training/articles/perf-jni#javavm-and-jnienvb
    java_vm: JavaVM,
}

impl MyCallback for MyCallbackImpl {
    fn call(&self, par: String) {
        let env = self.java_vm.attach_current_thread().unwrap();

        let str = env.new_string(par)
            .expect("Couldn't create java string!");
        let str_j_value = JValue::from(JObject::from(str));

        env.call_method(
            self.callback.as_obj(),
            "call",
            "(Ljava/lang/String;)V",
            &[str_j_value],
        ).expect("Couldn't call callback");
    }
}
