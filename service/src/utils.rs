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

    port.parse()
        .expect(format!("cannot parse port: {port}").as_str())
}

pub fn is_traced() -> bool {
    std::env::var("TRACING").is_ok() || std::env::args().any(|e| e == "--trace")
}

// these apis will be read from the db going forward, like SQLite
const APIS: &str = include_str!("../apis.json");

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct APIResponse {
    pub success: bool,
    pub data: serde_json::Value,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct API {
    pub method: String,
    pub path: String,
    pub response: APIResponse,
    pub wait: Option<u64>,
}

pub struct APIs(Vec<API>);

impl APIs {
    pub fn response(&self, method: &str, path: &str) -> Option<APIResponse> {
        match self
            .0
            .iter()
            .find(|api| api.method.eq(method) && api.path.eq(path))
        {
            Some(api) => {
                if let Some(wait) = api.wait {
                    std::thread::sleep(std::time::Duration::from_millis(wait));
                }
                Some(api.response.clone())
            }
            None => None,
        }
    }
}

pub fn apis() -> Result<APIs, Box<dyn std::error::Error>> {
    let apis: Vec<API> = serde_json::from_str(APIS)?;
    Ok(APIs(apis))
}
