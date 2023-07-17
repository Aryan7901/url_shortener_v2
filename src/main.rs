use axum::{
    http::Method,
    routing::{get, post},
     Router,
};
use mongodb::{ options::ClientOptions, Client};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{CorsLayer, Origin};
use url_shortener::{create_short_url, get_url,AppState};
#[tokio::main]
async fn main() {
    let db_str=option_env!("DB").unwrap_or("mongodb://localhost:27017");
    let port = option_env!("PORT").unwrap_or("8000");
    let server=option_env!("SERVER").unwrap_or("http://localhost");
    let client_options = ClientOptions::parse(db_str).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("shortener");
    println!("db {}, port {} server {}",db_str,port,server);
    println!("Connected to mongodb");
    let collection = db.collection("urls");
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Origin::exact("http://localhost:3000".parse().unwrap()))
        .allow_origin(Origin::exact(
            "https://aryan7901.github.io".parse().unwrap(),
        ));
    let app_state = Arc::new(AppState { collection,server });
    let app = Router::new()
        .route("/:short_url_id", get(get_url))
        .route("/url", post(create_short_url))
        .with_state(app_state.clone())
        .layer(cors);
    let addr = SocketAddr::from(([127, 0, 0, 1], port.parse::<u16>().unwrap_or(8000)));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

