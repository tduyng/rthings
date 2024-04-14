use chrono::Local;

pub fn now() -> String {
    Local::now().to_rfc2822()
}
