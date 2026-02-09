use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing::{get, post},
    Router,
};
use mongodb::{options::ClientOptions, Client};
use std::env;
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{CorsLayer, Origin};
use url_shortener::{create_short_url, get_url, AppState};

#[tokio::main]
async fn main() {
    let db_str: String = env::var("DB").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let port: String = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let server = env::var("SERVER").unwrap_or_else(|_| "http://localhost".to_string());
    let client_options = ClientOptions::parse(db_str).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("shortener");
    println!("Connected to mongodb");
    let collection = db.collection("urls");
    let origins = vec![
        "http://localhost:3000".parse().unwrap(),
        "https://aryan7901.github.io".parse().unwrap(),
    ];
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Origin::list(origins))
        .allow_headers([CONTENT_TYPE]);
    let app_state = Arc::new(AppState { collection, server:server });
    let app = Router::new()
        .route("/:short_url_id", get(get_url))
        .route("/url", post(create_short_url))
        .with_state(app_state.clone())
        .layer(cors);
    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port.parse::<u16>().unwrap_or(8000)));
    println!("🚀 Server starting on: http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
