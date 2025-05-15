#[derive(PartialEq, Eq)]
pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

use crate::Security::*;

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    if security_level == Message {
        return server.expect("ERROR: program stops").to_string();
    } else if security_level == Unknown {
        return server.unwrap_or("").to_string();
    } else if security_level == Warning {
        return server.expect("WARNING: check the server").to_string();
    } else if security_level == NotFound {
        server
            .map(String::from)
            .unwrap_or_else(|f| format!("Not found: {}", f))
    } else {
        server.expect_err("ERROR: program stops").to_string()
    }
}
