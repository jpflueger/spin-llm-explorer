use anyhow::Result;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

// constants for inferencing option defaults
const DEFAULT_MAX_TOKENS: u32 = 75;
const DEFAULT_REPEAT_PENALTY: f32 = 1.1;
const DEFAULT_REPEAT_PENALTY_LAST_N_TOKEN_COUNT: u32 = 64;
const DEFAULT_TEMPERATURE: f32 = 0.0;
const DEFAULT_TOP_K: u32 = 40;
const DEFAULT_TOP_P: f32 = 0.9;

// api data model for inferencing
#[derive(Debug, Serialize, Deserialize)]
pub struct InferRequest {
    pub model: String,
    pub prompt: String,
    pub options: Option<InferRequestOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InferRequestOptions {
    pub max_tokens: Option<u32>,
    pub repeat_penalty: Option<f32>,
    pub repeat_penalty_last_n_token_count: Option<u32>,
    pub temperature: Option<f32>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
}

impl Default for InferRequestOptions {
    fn default() -> Self {
        InferRequestOptions {
            max_tokens: Some(DEFAULT_MAX_TOKENS),
            repeat_penalty: Some(DEFAULT_REPEAT_PENALTY),
            repeat_penalty_last_n_token_count: Some(DEFAULT_REPEAT_PENALTY_LAST_N_TOKEN_COUNT),
            temperature: Some(DEFAULT_TEMPERATURE),
            top_k: Some(DEFAULT_TOP_K),
            top_p: Some(DEFAULT_TOP_P),
        }
    }
}

impl Into<spin_sdk::llm::InferencingParams> for InferRequestOptions {
    fn into(self) -> spin_sdk::llm::InferencingParams {
        spin_sdk::llm::InferencingParams {
            //TODO: set constants for these defaults
            max_tokens: self.max_tokens.unwrap_or(DEFAULT_MAX_TOKENS),
            repeat_penalty: self.repeat_penalty.unwrap_or(DEFAULT_REPEAT_PENALTY),
            repeat_penalty_last_n_token_count: self.repeat_penalty_last_n_token_count.unwrap_or(DEFAULT_REPEAT_PENALTY_LAST_N_TOKEN_COUNT),
            temperature: self.temperature.unwrap_or(DEFAULT_TEMPERATURE),
            top_k: self.top_k.unwrap_or(DEFAULT_TOP_K),
            top_p: self.top_p.unwrap_or(DEFAULT_TOP_P),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InferResponse {
    pub text: String,
    pub usage: InferResponseUsage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InferResponseUsage {
    pub prompt_token_count: u32,
    pub generated_token_count: u32,
}

impl From<spin_sdk::llm::InferencingResult> for InferResponse {
    fn from(result: spin_sdk::llm::InferencingResult) -> Self {
        InferResponse {
            text: result.text,
            usage: InferResponseUsage {
                prompt_token_count: result.usage.prompt_token_count,
                generated_token_count: result.usage.generated_token_count,
            },
        }
    }
}

/*

Ideas:
- list the supported models somewhere on the page
- prefill a list of prompts from ai.meta
- allow the user to set the different fields of the inferencing params
 */

#[http_component]
fn handle_api(req: Request) -> Result<Response> {
    // parse the request
    let request: InferRequest = serde_json::from_slice(req.body().as_ref().unwrap_or(&Bytes::new()))?;
    let model: spin_sdk::llm::InferencingModel = match request.model.as_str() {
        "llama2-chat" => spin_sdk::llm::InferencingModel::Llama2Chat,
        "llama2-code" => spin_sdk::llm::InferencingModel::CodellamaInstruct,
        m => spin_sdk::llm::InferencingModel::Other(m),
    };
    let options: spin_sdk::llm::InferencingParams = request.options.unwrap_or_default().into();

    // run the inference
    let inferred_result = spin_sdk::llm::infer_with_options(
        model,
        &request.prompt,
        options,
    );

    match inferred_result {
        Ok(result) => Ok(http::Response::builder()
            .status(200)
            .header("Content-Type", "application/json; charset=utf-8")
            .body(Some(serde_json::to_string(&InferResponse::from(result))?.into()))?),

        //TODO: handle errors better
        Err(e) => Ok(http::Response::builder()
            .status(500)
            .body(Some(e.to_string().into()))?),
    }
}
