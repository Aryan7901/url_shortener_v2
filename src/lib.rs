use std::sync::Arc;

use axum::{extract::{State, Path}, response::Redirect, http::StatusCode, Json};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub struct AppState<'a> {
    pub collection: mongodb::Collection<ShortUrl>,
    pub server: &'a str,
}
#[derive(Deserialize)]
pub struct CreateShortUrl {
    #[serde(rename(serialize = "shortUrl"))]
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ShortUrl {
    #[serde(rename(serialize = "longURL", deserialize = "longURL"))]
    pub long_url: String,
    #[serde(rename(serialize = "shortURL", deserialize = "shortURL"))]
    pub short_url: String,
    #[serde(rename(serialize = "shortUrlId", deserialize = "shortUrlId"))]
    pub short_url_id: String,
}

pub async fn get_url(
    State(state): State<Arc<AppState<'_>>>,
    Path(short_url_id): Path<String>,
) -> Result<Redirect, (StatusCode, String)> {
    let filter = doc! {"shortUrlId":short_url_id};
    let future = state.collection.find_one(filter, None).await;
    if future.is_err() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Some error occured".to_owned(),
        ));
    };
    match future.unwrap() {
        Some(result) => Ok(Redirect::temporary(&result.long_url)),
        None => Err((StatusCode::BAD_REQUEST, "Invalid URL".to_owned())),
    }
}

pub async fn create_short_url(
    State(state): State<Arc<AppState<'_>>>,
    Json(url_body): Json<CreateShortUrl>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let short_id = nanoid::nanoid!(9);
    let short_url = format!("{}/{}",state.server, short_id);
    let doc = ShortUrl {
        short_url:short_url.clone(),
        long_url: url_body.url,
        short_url_id: short_id,
    };
    match state.collection.insert_one( doc, None).await {
        Ok(_) => Ok(Json(json!({"shortUrl":short_url}))),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Your request failed, please try again!".to_owned(),
        )),
    }
}
