use rocket::State;
use rocket::{
    data::{Data, ToByteUnit},
    futures::future::MapErr,
};

use deadpool_redis::Pool;
use rocket_contrib;
use rocket_contrib::json::Json;

use redisai::model::{AIModel, AIModelMeta};
use redisai::{AIDataType, Backend, Device, RedisAIClient};
use shared::{ApiAIModel, ModelsApiResponse};
use std::str::FromStr;

// use rocket_okapi::{openapi, JsonSchema};
// use serde::{Deserialize, Serialize};

use crate::error::ApiError;

#[get("/models/list")]
pub async fn list_models(
    pool: State<'_, Pool>,
    redis_ai_client: State<'_, RedisAIClient>,
) -> Result<Json<ModelsApiResponse>, ApiError> {
    let mut conn = pool.get().await?;

    let mut models: Vec<ApiAIModel> = vec![];
    let models_redis: Vec<Vec<String>> = redis_ai_client.ai_modelscan_async(&mut conn).await?;

    if models_redis.len() == 0 {
        return Err(ApiError::EmptyModelError(Json(
            String::from("The model list in redis is empty").into(),
        )));
    }

    for model in models_redis.into_iter() {
        let one = ApiAIModel {
            model_name: model[0].clone(),
            tag: model[1].clone(),
            backend: Backend::from_str("ONNX").unwrap(),
            dtype: AIDataType::from_str("FLOAT").unwrap(),
            device: Device::CPU,
        };
        models.push(one);
    }
    let response = ModelsApiResponse { data: models };
    Ok(Json(response))
}
#[get("/models/<key>")]
pub async fn get_model(
    pool: State<'_, Pool>,
    redis_ai_client: State<'_, RedisAIClient>,
    key: String,
) -> Result<Json<ApiAIModel>, ApiError> {
    let mut conn = pool.get().await?;
    let models_redis: AIModel = redis_ai_client
        .ai_modelget_async(&mut conn, key.clone(), true)
        .await?;
    // match models_redis {
    //     Ok(model_query) =>
    // }
    Ok(Json(ApiAIModel {
        model_name: key.clone(),
        tag: models_redis.meta.tag.unwrap().clone(),
        backend: models_redis.meta.backend,
        dtype: AIDataType::from_str("FLOAT").unwrap(),
        device: models_redis.meta.device,
    }))
}

// #[post("/admin/models", data = "<ml_model>")]
// pub async fn upload_model(
//     ml_model: Data,
//     pool: State<'_, Pool>,
//     redis_ai_client: State<'_, RedisAIClient>,
// ) -> Result<(), DataStoreError> {
//     let mut conn = pool.get().await.unwrap();
//     let model_name = "models:wav2vec:v1";
//     let meta = AIModelMeta::default();
//     let ai_model = AIModel {
//         meta,
//         blob: ml_model
//             .open(2.mebibytes())
//             .into_bytes()
//             .await
//             .unwrap()
//             .value,
//     };
//     redis_ai_client
//         .ai_modelset_async(&mut conn, model_name.to_string(), ai_model)
//         .await
//         .unwrap();

//     Ok(())
// }
