use anyhow::{anyhow, Result};
use serde::{Serialize, Deserialize};
use spin_sdk::{http::{Request, Response}, llm::{InferencingModel, InferencingParams, InferencingResult}};

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub model: Option<String>,
    pub messages: Vec<GenerationMessage>,
    pub options: Option<GenerationModelOptions>,
}

impl GenerationRequest {
  // there is probably a more idiomatic way to do this
  // but I'm tired of fighting the borrow checker
  pub fn get_model(&self) -> Result<InferencingModel> {
    match self.model.as_deref() {
      Some("llama2-chat") => Ok(InferencingModel::Llama2Chat),
      Some("codellama-instruct") => Ok(InferencingModel::CodellamaInstruct),
      Some(other) => Ok(InferencingModel::Other(other)),
      None => Err(anyhow!("No model specified")),
    }
  }

  // there is probably a more idiomatic way to do this
  // but I'm tired of fighting the borrow checker
  pub fn get_params(&self) -> Result<InferencingParams> {
      let defaults = InferencingParams::default();
      match self.options {
        Some(ref options) => {
          Ok(InferencingParams {
            max_tokens: options.max_tokens.unwrap_or(defaults.max_tokens),
            repeat_penalty: options.repeat_penalty.unwrap_or(defaults.repeat_penalty),
            repeat_penalty_last_n_token_count: options.repeat_penalty_last_n_token_count.unwrap_or(defaults.repeat_penalty_last_n_token_count),
            temperature: options.temperature.unwrap_or(defaults.temperature),
            top_k: options.top_k.unwrap_or(defaults.top_k),
            top_p: options.top_p.unwrap_or(defaults.top_p),
          })
        },
        None => Ok(InferencingParams::default()),
      }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationMessage {
  pub role: String,
  pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationModelOptions {
    pub max_tokens: Option<u32>,
    pub repeat_penalty: Option<f32>,
    pub repeat_penalty_last_n_token_count: Option<u32>,
    pub temperature: Option<f32>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationResponse {
    pub messages: Vec<GenerationMessage>,
    pub usage: GenerationUsage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationUsage {
    pub prompt_token_count: u32,
    pub generated_token_count: u32,
}

pub struct GenerationRequestBuilder {
  req: Request,
}
impl GenerationRequestBuilder {
  pub fn new(req: Request) -> Self {
    Self { req }
  }

  pub fn build(self) -> Result<GenerationRequest> {
    let body = match self.req.body() {
      Some(b) => b.to_vec(),
      None => Default::default(),
    };
    let request: GenerationRequest = serde_json::from_slice(&body)?;

    //TODO: add behaviors here:
    // - read options from config variables
    // - read options from key-value store
    // - override options from request
    // - validation

    Ok(request)
  }
}

pub struct GenerationResponseBuilder {
  res: InferencingResult,
}
impl GenerationResponseBuilder {
  pub fn new(res: InferencingResult) -> Self {
    Self { res }
  }

  pub fn build(self) -> Result<Response> {
    let response = GenerationResponse {
      messages: vec![GenerationMessage {
        role: "system".to_string(),
        content: self.res.text,
      }],
      usage: GenerationUsage {
        prompt_token_count: self.res.usage.prompt_token_count,
        generated_token_count: self.res.usage.generated_token_count,
      },
    };
    let body = serde_json::to_vec(&response)?;
    Ok(http::Response::builder()
      .status(200)
      .body(Some(body.into()))
      .unwrap())
  }
}
