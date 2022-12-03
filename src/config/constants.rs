use std::env;
pub struct Env {
    pub zeromq_port: String,
    pub graphql_host: String,
    pub graphql_endpoint: String,
    pub graphql_port: String,
    pub rust_quiet: bool,
}

fn create_url() -> String {
    let port = env::var("GRAPHQL_PORT").unwrap_or("8000".to_string());
    let rust_env = env::var("RUST_ENV").unwrap_or("development".to_string());
    match env::var("GRAPHQL_HOST"){
        Ok(v) => {
            if v == "localhost".to_string() && rust_env == "production".to_string() {
                return format!("0.0.0.0:{}", port);
            } else {
                return format!("127.0.0.1:{}", port);
            }
        },
        Err(e) =>return format!("127.0.0.1:{}", port),
    }
}

fn get_bool(env_var: &String) -> bool {
    match env::var(env_var){
        Ok(v) => {
            if v == "false".to_string() || v == "".to_string() {
                return false;
            } else {
                return true;
            }
        },
        Err(e) => return false,
    }

}
pub fn get_env() -> Env {
    return Env {
        zeromq_port: env::var("ZEROMQ_PORT").unwrap_or("5555".to_string()),
        graphql_host: env::var("GRAPHQL_HOST").unwrap_or("localhost".to_string()),
        graphql_endpoint: create_url(),
        graphql_port: env::var("GRAPHQL_PORT").unwrap_or("8000".to_string()),
        rust_quiet: get_bool(&"RUST_QUIET".to_string())
    };
}
