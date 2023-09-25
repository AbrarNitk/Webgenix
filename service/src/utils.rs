pub fn read_env() -> String {
    match std::env::var("ENV") {
        Ok(env) => env.to_lowercase(),
        Err(_) => "local".to_string(),
    }
}

pub fn read_port_env() -> u16 {
    let port = match std::env::var("PORT") {
        Ok(env) => env.to_lowercase(),
        Err(_) => "8000".to_string(),
    };

    port.parse().expect(format!("cannot parse port: {port}").as_str())
}

pub fn is_traced() -> bool {
    std::env::var("TRACING").is_ok() || std::env::args().any(|e| e == "--trace")
}
