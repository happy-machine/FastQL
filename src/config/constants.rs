use std::env;
pub struct Env {
    pub zeromq_port: String,
    pub graphql_host: String,
    pub graphql_endpoint: String,
    pub graphql_port: String,
    pub rust_quiet: bool,
    pub tracing: bool,
    pub enable_graphiql: bool,
    pub cors_permissive: bool,
    pub allowed_origin_header: String,
    pub max_age_header: usize,
    pub allow_authorization_header: bool,
    pub allow_content_type_header: bool,
}

fn create_url() -> String {
    let port = env::var("GRAPHQL_PORT").unwrap_or("8000".to_string());
    let rust_env = env::var("RUST_ENV").unwrap_or("development".to_string());
    match env::var("GRAPHQL_HOST"){
        Ok(v) => {
            if v == "localhost".to_string() && rust_env == "production".to_string() {
                return format!("0.0.0.0:{}", port);
            } else if  v == "localhost".to_string() {
                return format!("127.0.0.1:{}", port);
            } else {
                return v;
            }
        },
        Err(e) => return format!("127.0.0.1:{}", port),
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


fn get_bool_default_true(env_var: &String) -> bool {
    match env::var(env_var){
        Ok(v) => {
            if v == "false".to_string() {
                return false;
            } else {
                return true;
            }
        },
        Err(e) => return true,
    }
}

fn set_graphiql() -> bool {
    match env::var("ENABLE_GRAPHIQL"){
        Ok(v) => {
            if v == "false".to_string() {
                return false;
            } else if v == "true".to_string() {
                return true;
            } else {
                return true;
            }
        },
        Err(e) => {
            if env::var("RUST_ENV").unwrap_or("development".to_string()) == "production".to_string() {
                return false;
            } else {
                return true;
            }
        },
    };
}

pub fn get_env() -> Env {
    return Env {
        zeromq_port: env::var("ZEROMQ_PORT").unwrap_or("5555".to_string()),
        graphql_host: env::var("GRAPHQL_HOST").unwrap_or("localhost".to_string()),
        graphql_endpoint: create_url(),
        graphql_port: env::var("GRAPHQL_PORT").unwrap_or("8000".to_string()),
        rust_quiet: get_bool(&"RUST_QUIET".to_string()),
        tracing: get_bool(&"TRACING".to_string()),
        enable_graphiql: set_graphiql(),
        cors_permissive: get_bool_default_true(&"CORS_PERMISSIVE".to_string()),
        allowed_origin_header: env::var("ALLOWED_ORIGIN_HEADER").unwrap_or("*".to_string()),
        max_age_header: env::var("MAX_AGE_HEADER").unwrap_or("3600".to_string()).parse().unwrap(),
        allow_authorization_header: get_bool(&"ALLOW_AUTHORIZATION_HEADER".to_string()),
        allow_content_type_header: get_bool(&"ALLOW_CONTENT_TYPE_HEADER".to_string()),
    }; 
}
