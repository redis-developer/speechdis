#[macro_use]
extern crate rocket;
use rocket::State;

use rocket_contrib::json::Json;

use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, routes_with_openapi, JsonSchema};

use redisai;
use serde::{Deserialize, Serialize};

use deadpool_redis::{cmd, Pool};

mod admin;

// use std::io::Cursor;

// use rocket::http::ContentType;
// use rocket::request::Request;
// use rocket::response::{self, Responder, Response};
// use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum DataStoreError {
//     #[error("data store disconnected")]
//     Disconnect(#[from] std::io::Error),
//     #[error("the data for key `{0}` is not available")]
//     Redaction(String),
// }

// impl<'r> Responder<'r, 'static> for DataStoreError {
//     fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
//         match self {
//             DataStoreError::Disconnect(_) => {
//                 let person_string = format!("data store disconnected");
//                 Response::build()
//                     .sized_body(person_string.len(), Cursor::new(person_string))
//                     .header(ContentType::new("application", "x-db"))
//                     .ok()
//             }
//             DataStoreError::Redaction(s) => {
//                 let person_string = format!("the data for key  is not available");
//                 Response::build()
//                     .sized_body(person_string.len(), Cursor::new(person_string))
//                     .header(ContentType::new("application", "x-db"))
//                     .ok()
//             }
//         }
//     }
// }
struct AppInfo {
    app_name: &'static str,
    version: &'static str,
    start_at: std::time::Instant,
}
#[derive(Serialize, Deserialize, JsonSchema)]
struct Healthcheck {
    app_name: &'static str,
    version: &'static str,
    uptime: std::time::Duration,
    redis_ready: bool,
}

async fn create_pool() -> Pool {
    let mut cfg = deadpool_redis::Config::default();
    cfg.url = Some("redis://127.0.0.1:6379".to_string());
    let pool = cfg.create_pool().unwrap();
    pool
}

#[openapi]
#[get("/healthcheck")]
async fn handle_healthcheck(
    app_info: State<'_, AppInfo>,
    pool: State<'_, Pool>,
) -> Json<Healthcheck> {
    let mut conn = pool.get().await.unwrap();
    let reply: String = cmd("PING").query_async(&mut conn).await.unwrap();
    let redis_ready = match reply.as_str() {
        "PONG" => true,
        _ => false,
    };
    Json(Healthcheck {
        app_name: app_info.app_name,
        version: app_info.version,
        uptime: app_info.start_at.elapsed(),
        redis_ready: redis_ready,
    })
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "../openapi.json".to_string(),
        ..Default::default()
    }
}

#[launch]
async fn rocket() -> _ {
    let pool = create_pool().await;
    let app_info = AppInfo {
        app_name: "speechdis",
        version: std::env!("CARGO_PKG_VERSION"),
        start_at: std::time::Instant::now(),
    };
    let redis_ai_client = redisai::RedisAIClient { debug: true };
    rocket::build()
        .manage(app_info)
        .manage(pool)
        .manage(redis_ai_client)
        .mount(
            "/",
            routes_with_openapi![handle_healthcheck, admin::upload_model],
        )
        .mount("/docs/", make_swagger_ui(&get_docs()))
}
