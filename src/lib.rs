use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jstring};

mod utils {
    pub mod date_util;
    //pub use self::date_util::*;

    pub mod file_util;
    //pub use self::file_util::*;
}

use utils::date_util::{get_now_string, get_system_time_now_string, get_formatted_now_string};
use utils::file_util::{create_rust_log_dir, write_rust_log};

#[cfg(target_os="android")]
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_cn_com_company_rustonandroid_MainActivity_doSth<'local>(
    env: JNIEnv<'local>, _: JClass<'local>, input: JString<'local>) -> jstring {
    let java_string2rust_string: String = env.get_string(&input)
        .expect("could not get java string from kotlin").into();
    let java_string2rust_string: String = String::from("");
    unsafe {
        // better performance than get_string, but unsafe
        let java_string2rust_string : String = env.get_string_unchecked(&input).expect("could not get java string from kotlin").into();
        let strDateTime = get_now_string();
        let num = rand::thread_rng().gen_range(1000..9999);

        let output = env.new_string(format!("{}: {}; date from rust: {}",
            java_string2rust_string, num, strDateTime)).expect("could not create java string");

        create_rust_log_dir();

        let msg = format!("Android call end at: {} / {} / {}",
            get_system_time_now_string(), strDateTime, get_formatted_now_string("%Y/%m/%d %H:%M:%S"));
        write_rust_log(&msg);

        output.into_raw()
    }
}
/*
https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html
vi ~/.cargo/config
[target.aarch64-linux-android]
linker = "/root/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android29-clang"

[target.armv7-linux-androideabi]
linker = "/root/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi29-clang
"

[target.i686-linux-android]
linker = "/root/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android29-clang"

cargo build --target aarch64-linux-android --release
cargo build --target add armv7-linux-androideabi --release
cargo build --target i686-linux-android --release

warning: `rust-on-android` (lib) generated 2 warnings (run `cargo fix --lib -p rust-on-android` to apply 2 suggestions)
*/
