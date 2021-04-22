use rocket::data::{Data, ToByteUnit};
use rocket::State;

use rocket_contrib;
use rocket_contrib::json::Json;

use redisai::model::{AIModel, AIModelMeta};
use redisai::RedisAIClient;

use rocket_okapi::{openapi, JsonSchema};
use serde::{Deserialize, Serialize};

use deadpool_redis::Pool;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RedisAIModel {
    model_name: &'static str,
}

#[openapi]
#[post("/admin/model", data = "<ml_model>")]
pub async fn upload_model(
    ml_model: Data,
    pool: State<'_, Pool>,
    redis_ai_client: State<'_, RedisAIClient>,
) -> Json<RedisAIModel> {
    let mut conn = pool.get().await.unwrap();
    let model_name = "models:wav2vec:v1";
    let meta = AIModelMeta::default();
    let ai_model = AIModel {
        meta,
        blob: ml_model
            .open(2.mebibytes())
            .into_bytes()
            .await
            .unwrap()
            .value,
    };
    redis_ai_client
        .ai_modelset_async(&mut conn, model_name.to_string(), ai_model)
        .await
        .unwrap();

    Json(RedisAIModel {
        model_name: "models:wav2vec:v1",
    })
}
