use anyhow::{Result, bail};
use shared::{GenerationRequestBuilder, GenerationRequest, GenerationMessage, GenerationResponseBuilder};
use spin_sdk::{
    http::{Params, Request, Response, Router},
    http_component,
};

// constants for inferencing option defaults
// const DEFAULT_MAX_TOKENS: u32 = 75;
// const DEFAULT_REPEAT_PENALTY: f32 = 1.1;
// const DEFAULT_REPEAT_PENALTY_LAST_N_TOKEN_COUNT: u32 = 64;
// const DEFAULT_TEMPERATURE: f32 = 0.0;
// const DEFAULT_TOP_K: u32 = 40;
// const DEFAULT_TOP_P: f32 = 0.9;
// const DEFAULT_MODEL: &str = "llama2-chat";
// const DEFAULT_SYSTEM_PROMPT: &str = "You are a helpful, respectful and honest assistant. Always answer as helpfully as possible, while being safe.  Your answers should not include any harmful, unethical, racist, sexist, toxic, dangerous, or illegal content. Please ensure that your responses are socially unbiased and positive in nature.

// If a question does not make any sense, or is not factually coherent, explain why instead of answering something not correct. If you don't know the answer to a question, please don't share false information.";

#[http_component]
fn handle(req: Request) -> Result<Response> {
    let component_route = req
        .headers()
        .get("spin-component-route")
        .unwrap()
        .to_str()?;
    let mut router = Router::new();
    router.post(&format!("{}/completions", component_route), handle_completion);
    router.handle(req)
}

fn handle_completion(http_req: Request, _params: Params) -> Result<Response> {
  // parse the request
  let api_req = GenerationRequestBuilder::new(http_req).build()?;
  let model = api_req.get_model()?;
  let options = api_req.get_params()?;
  let prompt = get_completion_prompt(&api_req)?;

  let infer_result = spin_sdk::llm::infer_with_options(model, &prompt, options)?;

  GenerationResponseBuilder::new(infer_result).build()
}

fn get_completion_prompt(req: &GenerationRequest) -> Result<String> {
  if req.messages.len() == 0 {
    bail!("No messages provided");
  }

  let system_prompts: Vec<&GenerationMessage> = req.messages
    .iter()
    .filter(|p| p.role == "system")
    .collect();

  let user_prompts: Vec<&GenerationMessage> = req.messages
    .iter()
    .filter(|p| p.role == "user")
    .collect();

  if req.messages.len() > 2 || system_prompts.len() > 1 || user_prompts.len() > 1 {
    bail!("Only one system and one user message are supported");
  }

  let system_prompt = system_prompts
    .first()
    .expect("No system prompt provided");
  let user_prompt = user_prompts
    .first()
    .expect("No user prompt provided");

  let prompt = format!(
      "<s>[INST] <<SYS>>\n{}\n<</SYS>>\n\n{} [/INST]",
      system_prompt.content, user_prompt.content
  );
  Ok(prompt)
}
