use bytes::Bytes;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::Request,
    llm::{InferencingModel, InferencingParams, InferencingResult, InferencingUsage},
};
use strum::{AsRefStr, Display};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub model: Model,
    pub messages: Vec<Message>,
    pub user: Option<String>,
    pub max_tokens: Option<u32>,
    pub repeat_penalty: Option<f32>,
    pub repeat_penalty_last_n_token_count: Option<u32>,
    pub temperature: Option<f32>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub id: String,
    pub created: u64,
    pub content: String,
    pub usage: Usage,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Display, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum Model {
    #[default]
    Llama2Chat,
    CodellamaInstruct,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelOptions {
    pub max_tokens: u32,
    pub repeat_penalty: f32,
    pub repeat_penalty_last_n_token_count: u32,
    pub temperature: f32,
    pub top_k: u32,
    pub top_p: f32,
}

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Display, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum MessageRole {
    System,
    #[default]
    User,
    Assistant,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_token_count: u32,
    pub generated_token_count: u32,
}

impl From<Model> for InferencingModel<'_> {
    fn from(value: Model) -> Self {
        match value {
            Model::Llama2Chat => InferencingModel::Llama2Chat,
            Model::CodellamaInstruct => InferencingModel::CodellamaInstruct,
        }
    }
}

impl From<InferencingParams> for ModelOptions {
    fn from(value: InferencingParams) -> Self {
        ModelOptions {
            max_tokens: value.max_tokens,
            repeat_penalty: value.repeat_penalty,
            repeat_penalty_last_n_token_count: value.repeat_penalty_last_n_token_count,
            temperature: value.temperature,
            top_k: value.top_k,
            top_p: value.top_p,
        }
    }
}

impl Default for ModelOptions {
    fn default() -> Self {
        InferencingParams::default().into()
    }
}

impl From<ModelOptions> for InferencingParams {
    fn from(value: ModelOptions) -> Self {
        InferencingParams {
            max_tokens: value.max_tokens,
            repeat_penalty: value.repeat_penalty,
            repeat_penalty_last_n_token_count: value.repeat_penalty_last_n_token_count,
            temperature: value.temperature,
            top_k: value.top_k,
            top_p: value.top_p,
        }
    }
}

impl From<InferencingUsage> for Usage {
    fn from(value: InferencingUsage) -> Self {
        Self {
            prompt_token_count: value.prompt_token_count,
            generated_token_count: value.generated_token_count,
        }
    }
}

impl From<InferencingResult> for CompletionResponse {
    fn from(value: InferencingResult) -> Self {
        Self {
            id: ulid::Ulid::new().into(),
            //TODO(jpflueger): use chrono instead?
            created: std::time::UNIX_EPOCH
                .elapsed()
                .expect("failed to get system time")
                .as_secs(),
            content: value.text,
            usage: value.usage.into(),
        }
    }
}

impl TryFrom<Request> for CompletionRequest {
    type Error = serde_json::Error;

    fn try_from(req: Request) -> Result<Self, Self::Error> {
        let body = match req.body() {
            Some(b) => b.to_vec(),
            None => Default::default(),
        };
        let request: CompletionRequest = serde_json::from_slice(&body)?;
        Ok(request)
    }
}

impl TryFrom<CompletionResponse> for Bytes {
    type Error = anyhow::Error;

    fn try_from(value: CompletionResponse) -> Result<Self, Self::Error> {
        let x = serde_json::to_vec(&value)?;
        Ok(Bytes::from(x))
    }
}
