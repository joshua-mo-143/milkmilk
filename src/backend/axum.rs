use crate::setup::DeployOn;
use std::fs;
use std::io;

use crate::utils::Utils;

pub struct Axum;

impl Axum {
    pub fn bootstrap(workdir: String, deploy_on: DeployOn) -> io::Result<()> {
        let mut main = workdir.clone();
        main.push_str("/src/main.rs");

        let mut router = workdir;
        router.push_str("/src/router.rs");

        match deploy_on {
            DeployOn::DockerImage => {
                Utils::write_to_file(&router, AXUM_MAIN_FILE)
            .expect("Failed to write the Axum main file :(");
            }
            DeployOn::Shuttle => {
                Utils::write_to_file(&router, AXUM_MAIN_FILE_SHUTTLE)
            .expect("Failed to write the Axum main file :(");
            }
        };

        Utils::write_to_file(&router, AXUM_ROUTER_FILE)
            .expect("Failed to write the Axum router file :(");

        Ok(())
    }
}

const AXUM_MAIN_FILE: &str = r#"use axum::Server;
use dotenvy::dotenv;
use std::net::SocketAddr;

mod router;
use router::create_api_router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let postgres = dotenvy::var("DATABASE_URL").expect("No database URL was set!");

    let postgres = sqlx::Pool::connect(&postgres).await.unwrap();

    sqlx::migrate!()
        .run(&postgres)
        .await
        .expect("Migrations failed :(");

    let router = create_api_router(postgres);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap()
}"#;

const AXUM_MAIN_FILE_SHUTTLE: &str = r#"

#[shuttle_runtime::main]
async fn axum(
#[shuttle_secrets::Postgres] postgres: PgPool,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&postgres)
        .await
        .expect("Migrations failed") :(");

    let router = create_api_router(postgres);

    router.into()
}   
"#;

const AXUM_ROUTER_FILE: &str = r#"use axum::extract::{Path, State};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Deserialize, Serialize, FromRow)]
pub struct Record {
    id: i32,
    name: String,
    price: String,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct CreateItemRequest {
    name: String,
    price: String,
}

#[derive(Deserialize)]
pub struct PriceRequest {
    price: String,
}

#[derive(Clone)]
pub struct AppState {
    postgres: PgPool,
}

pub fn create_api_router(postgres: PgPool) -> Router {
    let state = AppState { postgres };

    let items_router = Router::new()
        .route("/", get(get_items))
        .route("/create", post(create_item))
        .route(
            "/:id",
            get(get_one_item).put(update_item_price).delete(delete_item),
        )
        .with_state(state);

    Router::new().nest("/items", items_router)
}

pub async fn get_items(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let query = sqlx::query_as::<_, Record>("SELECT * FROM items").fetch_all(&state.postgres);

    match query.await {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_one_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let query = sqlx::query_as::<_, Record>("SELECT * FROM items WHERE id = $1")
        .bind(id)
        .fetch_one(&state.postgres);

    match query.await {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_item(
    State(state): State<AppState>,
    Json(request): Json<CreateItemRequest>,
) -> Result<StatusCode, StatusCode> {
    let price_formatted = format!("{:.2}", request.price);

    let query = sqlx::query("INSERT INTO items (name, price) VALUES ($1, $2)")
        .bind(request.name)
        .bind(price_formatted)
        .execute(&state.postgres);

    match query.await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn update_item_price(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(price): Json<i32>,
) -> Result<StatusCode, StatusCode> {
    let query = sqlx::query("UPDATE items SET price = $1 where id = $2")
        .bind(price)
        .bind(id)
        .execute(&state.postgres)
        .await;
    match query {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let query = sqlx::query("DELETE FROM items WHERE id = $1")
        .bind(id)
        .execute(&state.postgres)
        .await;

    match query {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}"#;
