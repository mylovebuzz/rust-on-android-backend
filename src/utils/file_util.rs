use std::fs::{OpenOptions, create_dir};
use std::io::{Write};

static RUST_LOG_DIR_IN_ANDROID:  &str = "/storage/emulated/0/Download/rustlogdir/";
static RUST_LOG_FILE_IN_ANDROID: &str ="rust_log.txt";

pub fn create_rust_log_dir() {

    create_dir(RUST_LOG_DIR_IN_ANDROID).ok();
}

pub fn write_rust_log(msg: &str) {

   // let mut owned_string = RUST_LOG_DIR_IN_ANDROID.to_owned().push_str(RUST_LOG_FILE_IN_ANDROID);
    let full_file_str = format!("{}{}", RUST_LOG_DIR_IN_ANDROID, RUST_LOG_FILE_IN_ANDROID);
    let mut file = OpenOptions::new().create(true).append(true).open(full_file_str).unwrap();
   //     .open(owned_string).unwrap();
    let msg_with_new_line = format!("{}{}", msg, "\n");
    file.write(msg_with_new_line.as_bytes()).unwrap();
}
