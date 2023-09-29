use anyhow::Result;
use shared::{model::CompletionRequest, CompletionChain, KvModelOptionsDefaults};
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
  let cmpl_req = CompletionRequest::try_from(http_req)?;

  // build the chain
  let chain = {
    let mut chain = CompletionChain::new();
    chain.with_options(KvModelOptionsDefaults::new(false));
    //TODO: load history from sqlite
    //TODO: add example of context from vss
    chain
  };

  // execute the chain
  let cmpl_res = chain.exec(&cmpl_req)?;

  //convert into a response
  let body = cmpl_res.try_into()?;
  let http_res = http::Response::builder()
    .status(200)
    .body(Some(body))?;
  Ok(http_res)
}
