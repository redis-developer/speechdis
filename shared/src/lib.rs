// use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

// use redisai::model::{AIModel, AIModelMeta};
use redisai::{AIDataType, Backend, Device};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiAIModel {
    pub model_name: String,
    pub tag: String,
    pub backend: Backend,
    pub dtype: AIDataType,
    pub device: Device,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelsApiResponse {
    pub data: Vec<ApiAIModel>,
}
