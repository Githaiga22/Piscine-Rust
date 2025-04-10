pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_string(), 
        Security::Message => server.expect("ERROR: program stops").to_string(),
        Security::Warning => match server {
            Ok(file) => format!("{file}"),
            Err(_) => format!("WARNING: check the server"),
        },
        Security::NotFound => match server {
            Ok(file) => format!("{file}"),
            Err(e) => format!("Not found: {}", e),
        },
        Security::UnexpectedUrl => match server {
            Ok(file) => panic!("{file}"),
            Err(e) => e.to_string(),
        },
    }
}
