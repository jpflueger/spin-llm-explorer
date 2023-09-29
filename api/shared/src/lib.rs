use anyhow::Result;
use model::{CompletionRequest, CompletionResponse, Message, ModelOptions};
use prompts::{PromptFormatter, Llama2PromptFormatter};
use spin_sdk::llm::infer_with_options;

pub mod model;
pub mod prompts;
pub mod tracking;

/// .
pub trait ModelOptionsDefaults {
  fn get(&self, req: &CompletionRequest) -> ModelOptions;
}

/// .
pub trait CompletionResponseCache {
    fn check(&self, req: &CompletionRequest) -> Option<CompletionResponse>;
}

/// .
pub trait ContextRetriever {
  fn get(&self, messages: &[Message]) -> Option<String>;
}

pub struct CompletionChain {
  options: Option<Box<dyn ModelOptionsDefaults>>,
  cache: Option<Box<dyn CompletionResponseCache>>,
  retriever: Option<Box<dyn ContextRetriever>>,
  formatter: Option<Box<dyn PromptFormatter>>,
}

pub struct KvModelOptionsDefaults {
  ignore_request: bool,
}
impl KvModelOptionsDefaults {
  pub fn new(ignore_request: bool) -> Self {
    Self { ignore_request }
  }

  pub(self) fn val_into_u32(b: Vec<u8>) -> Option<u32> {
    b.try_into().map(u32::from_be_bytes).ok()
  }

  pub(self) fn val_into_f32(b: Vec<u8>) -> Option<f32> {
    b.try_into().map(f32::from_be_bytes).ok()
  }
}
impl ModelOptionsDefaults for KvModelOptionsDefaults {
    fn get(&self, req: &CompletionRequest) -> ModelOptions {
        let store = spin_sdk::key_value::Store::open_default()
            .expect("failed to open default key value store");

        // start with the SDK defaults
        let mut options = ModelOptions::default();

        // fill from key value
        if let Ok(val) = store.get("max_tokens") {
          options.max_tokens = Self::val_into_u32(val)
            .unwrap_or(options.max_tokens);
        }

        if let Ok(val) = store.get("repeat_penalty") {
          options.repeat_penalty = Self::val_into_f32(val)
            .unwrap_or(options.repeat_penalty);
        }

        if let Ok(val) = store.get("repeat_penalty_last_n_token_count") {
          options.repeat_penalty_last_n_token_count = Self::val_into_u32(val)
            .unwrap_or(options.repeat_penalty_last_n_token_count);
        }

        if let Ok(val) = store.get("temperature") {
          options.temperature = Self::val_into_f32(val)
            .unwrap_or(options.temperature);
        }

        if let Ok(val) = store.get("top_k") {
          options.top_k = Self::val_into_u32(val)
            .unwrap_or(options.top_k);
        }

        if let Ok(val) = store.get("top_p") {
          options.top_p = Self::val_into_f32(val)
            .unwrap_or(options.top_p);
        }

        // fill from request if allowed
        if !self.ignore_request {
          if let Some(val) = req.max_tokens {
            options.max_tokens = val;
          }
          if let Some(val) = req.repeat_penalty {
            options.repeat_penalty = val;
          }
          if let Some(val) = req.repeat_penalty_last_n_token_count {
            options.repeat_penalty_last_n_token_count = val;
          }
          if let Some(val) = req.temperature {
            options.temperature = val;
          }
          if let Some(val) = req.top_k {
            options.top_k = val;
          }
          if let Some(val) = req.top_p {
            options.top_p = val;
          }
        }

        options
    }
}

impl CompletionChain {
    pub fn new() -> Self {
      CompletionChain {
        options: None,
        cache: None,
        retriever: None,
        formatter: Some(Box::new(Llama2PromptFormatter::default()))
      }
    }

    pub fn with_options(&mut self, options: impl ModelOptionsDefaults + 'static) {
      self.options = Some(Box::new(options))
    }

    pub fn with_cache(&mut self, cache: impl CompletionResponseCache + 'static) {
      self.cache = Some(Box::new(cache))
    }

    pub fn with_context(&mut self, retriever: impl ContextRetriever + 'static) {
      self.retriever = Some(Box::new(retriever))
    }

    pub fn with_formatter(&mut self, formatter: impl PromptFormatter + 'static) {
      self.formatter = Some(Box::new(formatter))
    }

    pub fn exec(&self, req: &CompletionRequest) -> Result<CompletionResponse> {
      if let Some(cached_response) = self.cache.as_ref().and_then(|c| c.check(req)) {
        return Ok(cached_response);
      }

      let formatter = self.formatter.as_ref().expect("formatter is required");

      //TODO(jpflueger): can we do this without cloning?
      let model = req.model.clone().into();

      // get the parameters allowing the options loader to use/ignore parameters from the request
      let params = self
        .options
        .as_ref()
        .map(|o| o.get(&req))
        .unwrap_or_default()
        .into();

      let prompt = if let Some(_) = self.retriever.as_ref().and_then(|r| r.get(&req.messages)) {
        anyhow::bail!("formatting with context is not yet supported")
      } else {
        anyhow::Ok(formatter.format(&req.messages))
      }?;

      let result = infer_with_options(model, &prompt, params)?;

      Ok(result.into())
    }
}
