use std::str::FromStr;

use serde::{Deserialize, Serialize};

use deadpool_redis::Pool;
use redisai::model::{AIModel, AIModelMeta};
use redisai::{AIDataType, Backend, Device, RedisAIClient};

use rocket::data::{Data, ToByteUnit};
use rocket::form::Form;
use rocket::http::ContentType;
use rocket::State;

use rocket_contrib::json::Json;

use rocket_multipart_form_data::mime;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions,
};

use crate::error::ApiError;
use shared::{ApiAIModel, ModelsApiResponse};

#[get("/models")]
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
    Ok(Json(ApiAIModel {
        model_name: key.clone(),
        tag: models_redis.meta.tag.unwrap().clone(),
        backend: models_redis.meta.backend,
        dtype: AIDataType::from_str("FLOAT").unwrap(),
        device: models_redis.meta.device,
    }))
}

// #[derive(Debug, Serialize, Deserialize, FromForm)]
// pub struct AIModelForm {
//     #[field(name = "model-name")]
//     pub model_name: String,
//     pub tag: String,
//     pub backend: String,
//     pub dtype: String,
//     pub device: String,
// }

#[post("/models", data = "<model>")]
pub async fn upload_model(content_type: &rocket::http::ContentType, model: Data) -> &'static str {
    let mut options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        // MultipartFormDataField::file("model-file")
        //     .content_type_by_string(Some(mime::APPLICATION_OCTET_STREAM))
        //     .unwrap(),
        MultipartFormDataField::text("model-name"),
        MultipartFormDataField::text("tag"),
        MultipartFormDataField::text("backend"),
        MultipartFormDataField::text("dtype"),
        MultipartFormDataField::text("device"),
    ]);

    let mut multipart_form_data = MultipartFormData::parse(content_type, model, options)
        .await
        .unwrap();

    //let photo = multipart_form_data.files.get("model-file"); // Use the get method to preserve file fields from moving out of the MultipartFormData instance in order to delete them automatically when the MultipartFormData instance is being dropped
    let model = multipart_form_data.files.remove("model-file");
    let name = multipart_form_data.texts.remove("model-name"); // Use the remove method to move text fields out of the MultipartFormData instance (recommended)
    let tag = multipart_form_data.texts.remove("tag");
    let backend = multipart_form_data.texts.remove("backend");
    let dtype = multipart_form_data.texts.remove("dtype");
    let device = multipart_form_data.texts.remove("device");

    if let Some(file_fields) = model {
        let file_field = &file_fields[0]; // Because we only put one "photo" field to the allowed_fields, the max length of this file_fields is 1.
        let _content_type = &file_field.content_type;
        let _file_name = &file_field.file_name;
        let _path = &file_field.path;

        dbg!(file_field);
        // You can now deal with the uploaded file.
    }

    if let Some(mut text_fields) = name {
        let text_field = text_fields.remove(0); // Because we only put one "text" field to the allowed_fields, the max length of this text_fields is 1.

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let text = text_field.text;

        // You can now deal with the text data.
        dbg!(&text);
    }
    // dbg!(&text_fields);
    // dbg!(&name);
    "ok"
}

// #[post("/models", data = "<model>")]
// pub async fn upload_model_meta(
//     model: Form<AIModelForm>,
//     pool: State<'_, Pool>,
//     redis_ai_client: State<'_, RedisAIClient>,
// ) -> Result<(), ApiError> {
//     let mut conn = pool.get().await?;
//     let key = format!(
//         "admin:models:{}:{}:{}:{}",
//         model.model_name.clone(),
//         model.backend.clone(),
//         model.device.clone(),
//         model.tag.clone()
//     );
//     let meta = AIModelMeta {
//         backend: Backend::from_str(&model.backend)?,
//         device: Device::from_str(&model.device)?,
//         // dtype: AIDataType::from_str(&model.dtype)?,
//         // TODO: WTF no dtype in model ???
//         tag: Some(model.tag.clone()),
//         ..Default::default()
//     };
//     let ai_model = AIModel { meta, blob: vec![] };
//     redis_ai_client
//         .ai_modelset_async(&mut conn, key, ai_model)
//         .await?;
//     Ok(())
// }

// #[post("/models/?<key>&<model>&<model_file>", data = "<model_file>")]
// pub async fn upload_model_file(
//     key: String,
//     model_file: Data,
//     pool: State<'_, Pool>,
//     redis_ai_client: State<'_, RedisAIClient>,
// ) -> Result<(), ApiError> {
//     let mut conn = pool.get().await?;
//     let model = redis_ai_client
//         .ai_modelget_async(&mut conn, key.clone(), true)
//         .await?;

//     let blob: Vec<u8> = model_file.open(2.mebibytes()).into_bytes().await?.value;

//     let ai_model = AIModel {
//         meta: model.meta,
//         blob,
//     };
//     redis_ai_client
//         .ai_modelset_async(&mut conn, key.clone(), ai_model)
//         .await?;

//     Ok(())
// }
