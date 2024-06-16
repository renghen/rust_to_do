mod config;

use axum::{routing::get, Router};
use config::Config;
use config_file::FromConfigFile;

use env_logger::Env;
use log::info;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World 5!" }));

    let env = Env::default()
        .filter_or("RUST_LOG", "info")
        .write_style_or("RUST_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let config = Config::from_config_file("config.toml").unwrap();
    info!("config {:?}", config);

    let socket = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&socket).await.unwrap();
    info!("listening to {}", socket);

    axum::serve(listener, app).await.unwrap();
}
