#[macro_use]
extern crate rocket;
use rocket::State;

use redisai;
use serde::{Deserialize, Serialize};

use deadpool_redis::{cmd, Pool};

use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::json::Json;

mod cors;
mod error;
mod routes;

use cors::CORS;
use error::ApiError;
use routes::admin;
use routes::predict;

async fn create_pool() -> Result<Pool, ApiError> {
    let mut cfg = deadpool_redis::Config::default();
    cfg.url = Some("redis://127.0.0.1:6379".to_string());
    let pool = cfg
        .create_pool()
        .expect("Cannot create a deadpool Redis Pool");
    Ok(pool)
}

pub struct AppInfo {
    app_name: &'static str,
    version: &'static str,
    start_at: std::time::Instant,
}
#[derive(Serialize, Deserialize)]
pub struct Healthcheck {
    app_name: &'static str,
    version: &'static str,
    uptime: std::time::Duration,
    redis_ready: bool,
}

#[get("/healthcheck")]
async fn handle_healthcheck(
    app_info: State<'_, AppInfo>,
    pool: State<'_, Pool>,
) -> Result<Json<Healthcheck>, ApiError> {
    let mut conn = pool.get().await?;
    let reply: String = cmd("PING").query_async(&mut conn).await?;
    let redis_ready = match reply.as_str() {
        "PONG" => true,
        _ => false,
    };
    Ok(Json(Healthcheck {
        app_name: app_info.app_name,
        version: app_info.version,
        uptime: app_info.start_at.elapsed(),
        redis_ready,
    }))
}

#[launch]
async fn rocket() -> _ {
    let pool = create_pool()
        .await
        .expect("cannot open connection to redis");
    let app_info = AppInfo {
        app_name: "speechdis",
        version: std::env!("CARGO_PKG_VERSION"),
        start_at: std::time::Instant::now(),
    };
    let redis_ai_client = redisai::RedisAIClient { debug: true };
    rocket::build()
        .attach(CORS)
        .attach(SpaceHelmet::default())
        .manage(app_info)
        .manage(pool)
        .manage(redis_ai_client)
        .mount("/api/v1", routes![handle_healthcheck])
        .mount(
            "/api/v1/admin",
            routes![
                admin::list_models,
                admin::get_model,
                admin::upload_model,
                // admin::upload_model_file
            ],
        )
}
