use anyhow::Result;
use shared::{GenerationRequest, GenerationResponseBuilder, InferSdkBuilder, llama2_prompt::Llama2PromptBuilder};
use spin_sdk::{
    http::{Params, Request, Response, Router},
    http_component,
};

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
  let api_req = GenerationRequest::try_from(http_req)?;

  let sdk_bldr = InferSdkBuilder::new()
    .with_model(api_req.model)
    .with_messages(api_req.messages)
    .with_params(api_req.params)
    //TODO: should be able to select this based on the model but I just want to get this done
    .with_prompt_builder(Box::new(Llama2PromptBuilder {}));

  let model = sdk_bldr.build_model()?;
  let prompt = sdk_bldr.build_prompt()?;
  let params = sdk_bldr.build_params();

  let infer_result = spin_sdk::llm::infer_with_options(model, &prompt, params)?;

  println!("{}, Formatted prompt: {:?}",
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
    prompt);

  GenerationResponseBuilder::new(infer_result).build()
}
