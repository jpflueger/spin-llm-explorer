use anyhow::{Context, Result};
use serde::Deserialize;
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

mod model {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Message {
        pub role: String,
        pub content: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Model {
        pub name: String,
        pub max_tokens: i32,
        pub temperature: f64,
        pub repeat_penalty: f64,
        pub repeat_penalty_last_n_tokens: i32,
        pub top_k: i32,
        pub top_p: f64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct App {
        pub id: i32,
        pub created_at: String,
        pub updated_at: String,
        pub name: String,
        pub description: String,
        pub messages: Vec<Message>,
        pub model: Model,
    }

    impl Default for Message {
        fn default() -> Self {
            Self {
                // TODO: set sensible defaults
                role: Default::default(),
                content: Default::default(),
            }
        }
    }

    impl Default for Model {
        fn default() -> Self {
            Self {
                // TODO: set sensible defaults
                name: Default::default(),
                max_tokens: Default::default(),
                temperature: Default::default(),
                repeat_penalty: Default::default(),
                repeat_penalty_last_n_tokens: Default::default(),
                top_k: Default::default(),
                top_p: Default::default(),
            }
        }
    }

    impl Default for App {
        fn default() -> Self {
            Self {
                id: Default::default(),
                created_at: Default::default(),
                updated_at: Default::default(),
                name: Default::default(),
                description: Default::default(),
                messages: vec![Default::default()],
                model: Default::default(),
            }
        }
    }
}

mod api {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct ListQuery {
        offset: Option<u64>,
        limit: Option<u64>,
    }

    pub fn list_apps(req: Request, _params: Params) -> Result<Response> {
        let list_query: ListQuery = serde_qs::from_str(req.uri().to_string().as_str())
            .context("failed to parse the request url")?;

        //TODO: clamp the values to something reasonable
        let offset = list_query.offset.unwrap_or(0);
        let limit = list_query.limit.unwrap_or(10);

        let apps = data::list(offset, limit)?;

        let body = serde_json::to_vec(&apps)?;

        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Some(body.into()))
            .unwrap())
    }

    pub fn get_app(_req: Request, params: Params) -> Result<Response> {
        let name = params
            .get("name")
            .context("url route missing name parameter.")?;

        let app = data::get_by_name(name.to_string())?;

        let body = serde_json::to_vec(&app)?;

        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Some(body.into()))
            .unwrap())
    }

    pub fn create_app(req: Request, _params: Params) -> Result<Response> {
        let req_body = match req.body() {
            Some(b) => b.to_vec(),
            None => Default::default(),
        };
        let new_app = serde_json::from_slice::<model::App>(&req_body)?;

        //TODO: validate the data

        //TODO: make sure the name is not taken

        let app = data::upsert(new_app)?;

        let resp_body = serde_json::to_vec(&app)?;

        Ok(http::Response::builder()
            .status(http::StatusCode::CREATED)
            .header(http::header::CONTENT_TYPE, "application/json")
            .header(http::header::LOCATION, format!("/api/app/{}", app.name))
            .body(Some(resp_body.into()))
            .unwrap())
    }

    pub fn update_app(req: Request, _params: Params) -> Result<Response> {
        let req_body = match req.body() {
            Some(b) => b.to_vec(),
            None => Default::default(),
        };
        let new_app = serde_json::from_slice::<model::App>(&req_body)?;

        //TODO: validate the data

        //TODO: make sure the name is not taken

        let app = data::upsert(new_app)?;

        let resp_body = serde_json::to_vec(&app)?;

        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Some(resp_body.into()))
            .unwrap())
    }

    pub fn delete_app(_req: Request, params: Params) -> Result<Response> {
        let name = params
            .get("name")
            .context("url route missing name parameter.")?;

        data::delete_by_name(name.to_string())?;

        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(None)
            .unwrap())
    }
}

mod data {
    use anyhow::Ok;

    use super::model::*;
    use super::*;

    pub fn list(_offset: u64, _limit: u64) -> Result<Vec<App>> {
        println!("data::list not implemented");
        Ok(vec![App {
            ..Default::default()
        }])
    }

    pub fn get_by_name(name: String) -> Result<App> {
        println!("data::get_by_name not implemented");
        Ok(App {
            name,
            ..Default::default()
        })
    }

    pub fn upsert(app: App) -> Result<App> {
        println!("data::upsert not implemented");
        Ok(app)
    }

    pub fn delete_by_name(_name: String) -> Result<()> {
        println!("data::delete_by_name not implemented");
        Ok(())
    }
}
