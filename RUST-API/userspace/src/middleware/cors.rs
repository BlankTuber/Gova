use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::env;

#[derive(Debug)]
struct ClientConfig {
    client_type: ClientType,
    allowed_methods: Vec<&'static str>,
}

#[derive(Debug, PartialEq)]
enum ClientType {
    Web,
    Game,
    Mobile,
    Desktop,
    Service,
}

impl From<&str> for ClientType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "web" => ClientType::Web,
            "game" => ClientType::Game,
            "mobile" => ClientType::Mobile,
            "desktop" => ClientType::Desktop,
            _ => ClientType::Service,
        }
    }
}

lazy_static! {
    static ref ROUTE_CONFIGS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("/api/auth", vec!["POST", "OPTIONS"]);
        m.insert("/api/admin", vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]);
        m.insert("/api/users", vec!["GET", "PUT", "OPTIONS"]);
        m
    };
}

pub struct CORS {
    client_origins: HashMap<String, ClientConfig>,
}

impl CORS {
    pub fn new() -> Self {
        let client_origins = Self::load_client_configs();
        CORS { client_origins }
    }

    fn load_client_configs() -> HashMap<String, ClientConfig> {
        let mut configs = HashMap::new();
        
        if let Ok(origins_str) = env::var("CLIENT_ORIGINS") {
            for origin_config in origins_str.split(',') {
                if let Some((origin, client_type)) = origin_config.split_once(':') {
                    let client_type = ClientType::from(client_type.trim());
                    let methods = Self::get_methods_for_client_type(&client_type);
                    
                    configs.insert(
                        origin.trim().to_string(),
                        ClientConfig {
                            client_type,
                            allowed_methods: methods,
                        },
                    );
                }
            }
        }

        // Add development origins if in debug mode
        if cfg!(debug_assertions) {
            configs.insert(
                "http://localhost:3000".to_string(),
                ClientConfig {
                    client_type: ClientType::Web,
                    allowed_methods: vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"],
                },
            );
            configs.insert(
                "http://127.0.0.1:3000".to_string(),
                ClientConfig {
                    client_type: ClientType::Web,
                    allowed_methods: vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"],
                },
            );
        }

        configs
    }

    fn get_methods_for_client_type(client_type: &ClientType) -> Vec<&'static str> {
        match client_type {
            ClientType::Web => vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"],
            ClientType::Game => vec!["GET", "POST", "OPTIONS"],
            ClientType::Mobile => vec!["GET", "POST", "PUT", "OPTIONS"],
            ClientType::Desktop => vec!["GET", "POST", "PUT", "OPTIONS"],
            ClientType::Service => vec!["GET", "POST", "OPTIONS"],
        }
    }

    fn get_headers_for_client_type(client_type: &ClientType) -> &'static str {
        match client_type {
            ClientType::Web => "Content-Type, Authorization, Accept, X-Real-IP, X-Requested-With",
            ClientType::Game => "Content-Type, Authorization, X-Game-Version, X-Game-Platform",
            ClientType::Mobile => "Content-Type, Authorization, X-App-Version, X-Device-Type",
            ClientType::Desktop => "Content-Type, Authorization, X-App-Version",
            ClientType::Service => "Content-Type, Authorization, X-Service-ID",
        }
    }

    fn is_origin_allowed(&self, origin: &str) -> Option<&ClientConfig> {
        self.client_origins.get(origin)
    }
}

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if let Some(origin) = request.headers().get_one("Origin") {
            if let Some(client_config) = self.is_origin_allowed(origin) {
                response.set_header(Header::new("Access-Control-Allow-Origin", origin));
                
                // Set methods based on client type and route
                let methods = client_config.allowed_methods.join(", ");
                response.set_header(Header::new("Access-Control-Allow-Methods", methods));
                
                // Set headers based on client type
                response.set_header(Header::new(
                    "Access-Control-Allow-Headers",
                    Self::get_headers_for_client_type(&client_config.client_type)
                ));

                response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

                if request.method() == rocket::http::Method::Options {
                    response.set_header(Header::new("Access-Control-Max-Age", "86400"));
                }
            }
        }
    }
}