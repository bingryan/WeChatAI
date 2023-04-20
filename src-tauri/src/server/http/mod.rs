use anyhow::Context;
use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::post,
    Router,
};
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::config::app::image_upload_path;
use crate::server::router::chatgpt as chatgptAPI;
use crate::server::router::image as imageAPI;

pub async fn serve() -> anyhow::Result<()> {
    let origins = [
        "tauri://localhost".parse::<HeaderValue>().unwrap(),
        "https://tauri.localhost".parse::<HeaderValue>().unwrap(),
        "http://tauri.localhost".parse::<HeaderValue>().unwrap(),
    ];
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true)
        .allow_headers([CONTENT_TYPE])
        .allow_origin(origins);

    // let cors = CorsLayer::permissive();

    let image_upload_path = image_upload_path().unwrap();
    let app = Router::new()
        .route("/chat-dev", post(chatgptAPI::create_chat_json))
        .route("/chat", post(chatgptAPI::create_chat_raw))
        .route("/upload/:file_name", post(imageAPI::upload))
        .nest_service("/assets/image", ServeDir::new(image_upload_path.as_path()))
        .layer(cors);

    axum::Server::bind(&"0.0.0.0:19999".parse()?)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}
