use anyhow::Result;
use spin_sdk::{
    http::{Params, Request, Response},
    http_component, http_router,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_app_api(req: Request) -> Result<Response> {
    // println!("{:?}", req.headers());

    let router = http_router! {
        GET "/api/app" => api::list_apps,
        GET "/api/app/:name" => api::get_app,
        POST "/api/app" => api::create_app,
        PUT "/api/app/:name" => api::update_app,
        DELETE "/api/app/:name" => api::delete_app,
        _ "/*" => |_req, _params| {
          Ok(http::Response::builder()
          .status(http::StatusCode::NOT_FOUND)
          .body(Some("Not found".into()))
          .unwrap())
        }
    };
    router.handle(req)
}

mod api {
    use super::*;

    pub fn list_apps(_req: Request, _params: Params) -> anyhow::Result<Response> {
        let body = "[
      {{
        id: 1,
        name: \"my-first-prompt-app\",
        description: \"This is my first prompt app!\",
        prompts: [{{
          type: \"system\",
          text: \"Welcome to your new prompt app!\",
        }}, {{
          type: \"user\",
          text: \"Hello, world!\",
        }}],
        model: {{
          name: \"llama2-chat\",
          options: {{
            max_tokens: 75,
            temperature: 0.0,
            repeat_penalty: 1.1,
            repeat_penalty_last_n_tokens: 64,
            top_k: 0,
            top_p: 1.0,
          }}
        }}
      }},
    ]";

        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Some(body.into()))
            .unwrap())
    }

    pub fn get_app(_req: Request, params: Params) -> anyhow::Result<Response> {
        let name = params.get("name").expect("Route missing name parameter.");

        let body = format!(
            "{{
        id: 1,
        name: \"{name}\",
        description: \"This is my first prompt app!\",
        prompts: [{{
          type: \"system\",
          text: \"Welcome to your new prompt app!\",
        }}, {{
          type: \"user\",
          text: \"Hello, world!\",
        }}],
        model: {{
          name: \"llama2-chat\",
          options: {{
            max_tokens: 75,
            temperature: 0.0,
            repeat_penalty: 1.1,
            repeat_penalty_last_n_tokens: 64,
            top_k: 0,
            top_p: 1.0,
          }}
        }}
      }}"
        );

        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Some(body.into()))
            .unwrap())
    }

    pub fn create_app(_req: Request, _params: Params) -> anyhow::Result<Response> {
        Ok(http::Response::builder()
            .status(http::StatusCode::CREATED)
            .body(Some("Created".into()))
            .unwrap())
    }

    pub fn update_app(_req: Request, _params: Params) -> anyhow::Result<Response> {
        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Some("Updated".into()))
            .unwrap())
    }

    pub fn delete_app(_req: Request, _params: Params) -> anyhow::Result<Response> {
        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Some("Deleted".into()))
            .unwrap())
    }
}
