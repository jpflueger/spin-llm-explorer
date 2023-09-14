use anyhow::{anyhow, Result, bail};
use serde::{Serialize, Deserialize};
use spin_sdk::{http::{Request, Response}, llm::{InferencingModel, InferencingParams, InferencingResult}};

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub model: Option<String>,
    pub messages: Vec<GenerationMessage>,
    pub params: Option<GenerationModelParams>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationModelParams {
  pub max_tokens: Option<u32>,
  pub repeat_penalty: Option<f32>,
  pub repeat_penalty_last_n_token_count: Option<u32>,
  pub temperature: Option<f32>,
  pub top_k: Option<u32>,
  pub top_p: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationMessage {
  pub role: String,
  pub content: String,
}

impl Clone for GenerationMessage {
  fn clone(&self) -> Self {
    Self {
      role: self.role.clone(),
      content: self.content.clone(),
    }
  }
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

// dat new new
impl TryFrom<Request> for GenerationRequest {
  type Error = anyhow::Error;

  fn try_from(req: Request) -> Result<Self> {
    let body = match req.body() {
      Some(b) => b.to_vec(),
      None => Default::default(),
    };
    let request: GenerationRequest = serde_json::from_slice(&body)?;
    Ok(request)
  }
}

pub trait PromptBuilder {
  fn build_prompt(&self, messages: Vec<GenerationMessage>) -> Result<String>;
}

pub mod llama2_prompt {
    // based on -> https://huggingface.co/TheBloke/Llama-2-7B-Chat-GGML/discussions/3

    /* example single prompt:
      <s>[INST] <<SYS>>
      You are a helpful, respectful and honest assistant. Always answer as helpfully as possible, while being safe.  Your answers should not include any harmful, unethical, racist, sexist, toxic, dangerous, or illegal content. Please ensure that your responses are socially unbiased and positive in nature.

      If a question does not make any sense, or is not factually coherent, explain why instead of answering something not correct. If you don't know the answer to a question, please don't share false information.
      <</SYS>>

      <prompt> [/INST]
    */

    /* example chat prompt:
      <s>[INST] <<SYS>>
      You are a helpful, respectful and honest assistant. Always answer as helpfully as possible, while being safe.  Your answers should not include any harmful, unethical, racist, sexist, toxic, dangerous, or illegal content. Please ensure that your responses are socially unbiased and positive in nature.

      If a question does not make any sense, or is not factually coherent, explain why instead of answering something not correct. If you don't know the answer to a question, please don't share false information.
      <</SYS>>

      <prompt> [/INST] <answer> </s><s>[INST] <prompt-second> [/INST]
    */

  use super::*;

  pub struct Llama2PromptBuilder {}

  fn format_usr(content: &str) -> String {
    format!("{} [/INST]\n\n", content)
  }

  fn format_asst(content: &str) -> String {
    format!("{} </s><s>[INST]\n\n", content)
  }

  fn format_sys(content: &str) -> String {
    format!(" <<SYS>>\n{}\n<</SYS>>\n\n", content)
  }

  impl PromptBuilder for Llama2PromptBuilder {
    fn build_prompt(&self, messages: Vec<GenerationMessage>) -> Result<String> {
      let mut prompt = String::new();

      for i in 0..messages.len() {
        let message = &messages[i];
        match (i, message.role.as_str()) {
          (0, "assistant") => bail!("First message cannot be from assistant"),
          (0, "system") => prompt.push_str(&format_sys(&message.content)),
          (_, "user") => prompt.push_str(&format_usr(&message.content)),
          (_, "assistant") => prompt.push_str(&format_asst(&message.content)),
          (_, "system") => bail!("System messages can only be the first message"),
          (_, _) => bail!("Invalid role"),
        }
      }

      Ok(prompt)
    }
  }
}

pub struct InferSdkBuilder {
  model: Option<String>,
  messages: Option<Vec<GenerationMessage>>,
  params: Option<GenerationModelParams>,
  prompt_builder: Option<Box<dyn PromptBuilder>>,
}
impl InferSdkBuilder {
  pub fn new() -> Self {
    Self {
      model: None,
      messages: None,
      params: None,
      prompt_builder: None
    }
  }

  pub fn with_model(self, model: Option<String>) -> Self {
    Self { model, ..self }
  }

  pub fn with_messages(self, messages: Vec<GenerationMessage>) -> Self {
    Self { messages: Some(messages), ..self }
  }

  pub fn with_params(self, params: Option<GenerationModelParams>) -> Self {
    Self { params, ..self }
  }

  pub fn with_prompt_builder(self, prompt_builder: Box<dyn PromptBuilder>) -> Self {
    Self { prompt_builder: Some(prompt_builder), ..self }
  }

  pub fn build_params(self) -> InferencingParams {
    let defaults = InferencingParams::default();
    match self.params {
      Some(p) => {
        InferencingParams {
          max_tokens: p.max_tokens.unwrap_or(defaults.max_tokens),
          repeat_penalty: p.repeat_penalty.unwrap_or(defaults.repeat_penalty),
          repeat_penalty_last_n_token_count: p.repeat_penalty_last_n_token_count.unwrap_or(defaults.repeat_penalty_last_n_token_count),
          temperature: p.temperature.unwrap_or(defaults.temperature),
          top_k: p.top_k.unwrap_or(defaults.top_k),
          top_p: p.top_p.unwrap_or(defaults.top_p),
        }
      },
      None => defaults,
    }
  }

  pub fn build_model(&self) -> Result<InferencingModel<'static>> {
    match self.model.as_deref() {
      Some("llama2-chat") => Ok(InferencingModel::Llama2Chat),
      Some("codellama-instruct") => Ok(InferencingModel::CodellamaInstruct),
      // I have fought with this for hours and I don't understand how to return the inferencing model enum since its a temporary value
      // other => Ok(InferencingModel::Other(other)),
      _ => Err(anyhow!("Support for other models is not yet implemented")),
    }
  }

  pub fn build_prompt(&self) -> Result<String> {
    match self.prompt_builder.as_deref() {
      Some(pb) => {
        match self.messages.as_deref() {
          Some(messages) => pb.build_prompt(messages.to_vec()),
          None => Err(anyhow!("Messages must be provided when using a prompt builder")),
        }
      },
      None => Err(anyhow!("Prompt builder must be provided")),
    }
  }
}
