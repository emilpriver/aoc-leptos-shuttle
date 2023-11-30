use dotenvy::dotenv;
use std::path::Path;

pub fn load_env_variables() {
    if Path::new(".env").exists() {
        dotenv().unwrap();
    }
}
