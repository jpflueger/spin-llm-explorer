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
    pub struct Model {
        pub name: String,
        pub max_tokens: u32,
        pub temperature: f64,
        pub repeat_penalty: f64,
        pub repeat_penalty_last_n_tokens: u32,
        pub top_k: u32,
        pub top_p: f64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct App {
        #[serde(default)]
        pub created_at: String,
        #[serde(default)]
        pub updated_at: String,
        pub name: String,
        #[serde(default)]
        pub description: String,
        pub system_prompt: String,
        pub model: Model,
    }

    impl Clone for Model {
        fn clone(&self) -> Self {
            Self {
                name: self.name.clone(),
                max_tokens: self.max_tokens.clone(),
                temperature: self.temperature.clone(),
                repeat_penalty: self.repeat_penalty.clone(),
                repeat_penalty_last_n_tokens: self.repeat_penalty_last_n_tokens.clone(),
                top_k: self.top_k.clone(),
                top_p: self.top_p.clone(),
            }
        }
    }

    impl Clone for App {
        fn clone(&self) -> Self {
            Self {
                created_at: self.created_at.clone(),
                updated_at: self.updated_at.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                system_prompt: self.system_prompt.clone(),
                model: self.model.clone(),
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
                created_at: Default::default(),
                updated_at: Default::default(),
                name: Default::default(),
                description: Default::default(),
                system_prompt: Default::default(),
                model: Default::default(),
            }
        }
    }
}

mod api {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct ListQuery {
        offset: Option<i64>,
        limit: Option<i64>,
    }

    pub fn list_apps(req: Request, _params: Params) -> Result<Response> {
        let list_query: ListQuery = serde_qs::from_str(req.uri().to_string().as_str())
            .context("failed to parse the request url")?;

        //TODO: clamp the values to something reasonable
        let offset = list_query.offset.unwrap_or(0);
        let limit = list_query.limit.unwrap_or(10);

        let db = data::Db::new()?;
        let apps = db.list(offset, limit)?;

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

        let db = data::Db::new()?;
        let app = db.get(name)?;

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

        let db = data::Db::new()?;
        db.insert(&new_app)?;
        let app = db.get(&new_app.name)?;

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

        let db = data::Db::new()?;
        db.update(&new_app)?;
        let app = db.get(&new_app.name)?;

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

        let db = data::Db::new()?;
        db.delete(&name)?;

        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(None)
            .unwrap())
    }
}

mod data {
    use std::collections::HashMap;

    use spin_sdk::sqlite::{Connection, ValueParam, ValueResult};

    use super::model::*;
    use super::*;

    pub struct Db {
        conn: Connection,
    }

    impl Db {
        pub fn new() -> Result<Self> {
            let conn = Connection::open_default()?;
            Ok(Self { conn })
        }

        pub fn list(&self, offset: i64, limit: i64) -> Result<Vec<App>> {
            let apps = self.select(
                "SELECT * FROM apps ORDER BY name ASC LIMIT ? OFFSET ?",
                &[ValueParam::Integer(limit), ValueParam::Integer(offset)],
            )?;

            Ok(apps)
        }

        pub fn get(&self, name: &str) -> Result<App> {
            let apps = self.select(
                "SELECT * FROM apps WHERE name = ?",
                &[ValueParam::Text(&name)],
            )?;

            if apps.len() == 0 {
                return Err(anyhow::anyhow!("App not found"));
            }

            // TODO: not sure how to handle this other than cloning?
            Ok(apps[0].clone())
        }

        pub fn insert(&self, app: &App) -> Result<()> {
            self.conn.execute(
                "INSERT INTO apps (
          name,
          description,
          system_prompt,
          model_name,
          model_max_tokens,
          model_temperature,
          model_repeat_penalty,
          model_repeat_penalty_last_n_tokens,
          model_top_k,
          model_top_p
        )
      VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                &[
                    ValueParam::Text(&app.name),
                    ValueParam::Text(&app.description),
                    ValueParam::Text(&app.system_prompt),
                    ValueParam::Text(&app.model.name),
                    ValueParam::Integer(app.model.max_tokens as i64),
                    ValueParam::Real(app.model.temperature),
                    ValueParam::Real(app.model.repeat_penalty),
                    ValueParam::Integer(app.model.repeat_penalty_last_n_tokens as i64),
                    ValueParam::Integer(app.model.top_k as i64),
                    ValueParam::Real(app.model.top_p),
                ],
            )?;

            Ok(())
        }

        pub fn update(&self, app: &App) -> Result<()> {
            self.conn.execute(
                "UPDATE apps SET
        description = ?,
        system_prompt = ?,
        model_name = ?,
        model_max_tokens = ?,
        model_temperature = ?,
        model_repeat_penalty = ?,
        model_repeat_penalty_last_n_tokens = ?,
        model_top_k = ?,
        model_top_p = ?
        WHERE name = ?",
                &[
                    ValueParam::Text(&app.description),
                    ValueParam::Text(&app.system_prompt),
                    ValueParam::Text(&app.model.name),
                    ValueParam::Integer(app.model.max_tokens as i64),
                    ValueParam::Real(app.model.temperature),
                    ValueParam::Real(app.model.repeat_penalty),
                    ValueParam::Integer(app.model.repeat_penalty_last_n_tokens as i64),
                    ValueParam::Integer(app.model.top_k as i64),
                    ValueParam::Real(app.model.top_p),
                    ValueParam::Text(&app.name),
                ],
            )?;
            Ok(())
        }

        pub fn delete(&self, name: &str) -> Result<()> {
            self.conn.execute(
                "DELETE FROM apps WHERE name = ?",
                &[ValueParam::Text(&name)],
            )?;
            Ok(())
        }

        fn select(&self, query: &str, params: &[ValueParam]) -> Result<Vec<App>> {
            let result = self.conn.execute(query, params)?;

            let col_map = Self::get_column_lookup(&result.columns);

            Ok(result
                .rows
                .iter()
                .flat_map(|r| -> Result<App, _> {
                    let created_at = r
                        .get::<&str>(col_map["created_at"])
                        .unwrap_or_default()
                        .to_string();
                    let updated_at = r
                        .get::<&str>(col_map["updated_at"])
                        .unwrap_or_default()
                        .to_string();
                    let name = r
                        .get::<&str>(col_map["name"])
                        .unwrap_or_default()
                        .to_string();
                    let description = r
                        .get::<&str>(col_map["description"])
                        .unwrap_or_default()
                        .to_string();
                    let system_prompt = r
                        .get::<&str>(col_map["system_prompt"])
                        .unwrap_or_default()
                        .to_string();
                    let model_name = r
                        .get::<&str>(col_map["model_name"])
                        .unwrap_or_default()
                        .to_string();
                    let model_max_tokens = r
                        .get::<u32>(col_map["model_max_tokens"])
                        .unwrap_or_default();
                    let model_repeat_penalty_last_n_tokens = r
                        .get::<u32>(col_map["model_repeat_penalty_last_n_tokens"])
                        .unwrap_or_default();
                    let model_top_k = r.get::<u32>(col_map["model_top_k"]).unwrap_or_default();

                    //TODO: once the llm-sdk branch is caught up with main we can use the float conversions from the sdk
                    let model_temperature = match r
                        .values
                        .get(col_map["model_temperature"])
                        .expect("missing expected column")
                    {
                        ValueResult::Real(f) => *f,
                        _ => 0.0,
                    };
                    let model_repeat_penalty = match r
                        .values
                        .get(col_map["model_repeat_penalty"])
                        .expect("missing expected column")
                    {
                        ValueResult::Real(f) => *f,
                        _ => 0.0,
                    };
                    let model_top_p = match r
                        .values
                        .get(col_map["model_top_p"])
                        .expect("missing expected column")
                    {
                        ValueResult::Real(f) => *f,
                        _ => 0.0,
                    };

                    anyhow::Ok(App {
                        created_at,
                        updated_at,
                        name,
                        description,
                        system_prompt: system_prompt,
                        model: Model {
                            name: model_name,
                            max_tokens: model_max_tokens,
                            temperature: model_temperature,
                            repeat_penalty: model_repeat_penalty,
                            repeat_penalty_last_n_tokens: model_repeat_penalty_last_n_tokens,
                            top_k: model_top_k,
                            top_p: model_top_p,
                        },
                    })
                })
                .collect())
        }

        fn get_column_lookup<'a>(columns: &'a Vec<String>) -> HashMap<&'a str, usize> {
            columns
                .iter()
                .enumerate()
                .map(|(i, c)| (c.as_str(), i))
                .collect::<HashMap<&str, usize>>()
        }
    }
}
