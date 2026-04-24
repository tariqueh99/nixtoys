use std::env;

pub fn get_display_session_protocol() -> String {
    match env::var("XDG_SESSION_TYPE") {
        Ok(value) => value,
        Err(e) => format!("Error: {}", e),
    }
}
