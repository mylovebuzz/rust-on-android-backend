//NG use std::time::SystemTime;
use chrono::Local;

//pub mod date_util {

    use std::time::SystemTime;
    pub fn get_system_time_now_string() -> String {

        let now  = SystemTime::now();
        format!("{:?}", now)
    }
//}

pub fn get_now_string() -> String {

    //let now = Local::now();
    //format!("{}", now)
    Local::now().to_string()
}

pub fn get_formatted_now_string(format: &str) -> String {
    Local::now().format(format).to_string()
}
