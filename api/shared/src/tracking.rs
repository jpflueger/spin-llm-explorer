use anyhow::{Result, bail};
use std::time::Duration;

use spin_sdk::{
    llm::{InferencingUsage, InferencingParams},
    sqlite::{Connection, ValueParam, ValueResult},
};

use crate::model::Message;

pub fn insert(
    conn: &Connection,
    formatted_prompt: &str,
    model_name: &str,
    params: &InferencingParams,
    usage: &InferencingUsage,
    duration: Duration,
    error: Option<&str>,
    messages: &[Message],
) -> Result<()> {
    let completion_id = insert_completion(
        conn,
        formatted_prompt,
        model_name,
        params,
        usage,
        duration,
        error,
    )?;
    insert_messages(conn, completion_id, messages)?;
    Ok(())
}

const INSERT_MESSAGE: &str = "INSERT INTO completion_messages (
  completion_id,
  role,
  content
) VALUES ";

fn insert_messages(
    conn: &Connection,
    completion_id: i64,
    messages: &[Message],
) -> Result<()> {
    let stmt = INSERT_MESSAGE.to_string()
        + (0..messages.len())
            .map(|_| "(?, ?, ?)")
            .collect::<Vec<_>>()
            .join(", ")
            .as_str();

    let params: Vec<ValueParam<'_>> = messages
        .iter()
        .flat_map(|m| {
            [
                ValueParam::Integer(completion_id),
                ValueParam::Text(m.role.as_ref()),
                ValueParam::Text(&m.content),
            ]
        })
        .collect();

    conn.execute(&stmt, &params)?;

    Ok(())
}

const INSERT_COMPLETION: &str = "INSERT INTO completions (
  formatted_prompt,
  model_name,
  max_tokens,
  temperature,
  repeat_penalty,
  repeat_penalty_last_n_tokens,
  top_k,
  top_p,
  prompt_token_count,
  generated_token_count,
  duration_ms,
  error
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id";

fn insert_completion(
    conn: &Connection,
    formatted_prompt: &str,
    model_name: &str,
    params: &InferencingParams,
    usage: &InferencingUsage,
    duration: Duration,
    error: Option<&str>,
) -> Result<i64> {
    let sql_params = &[
        ValueParam::Text(formatted_prompt),
        ValueParam::Text(model_name),
        ValueParam::Integer(params.max_tokens.into()),
        ValueParam::Real(params.temperature.into()),
        ValueParam::Real(params.repeat_penalty.into()),
        ValueParam::Integer(params.repeat_penalty_last_n_token_count.into()),
        ValueParam::Integer(params.top_k.into()),
        ValueParam::Real(params.top_p.into()),
        ValueParam::Integer(usage.prompt_token_count.into()),
        ValueParam::Integer(usage.generated_token_count.into()),
        ValueParam::Real(duration.as_secs_f64()), // might prefer .as_millis() here for simplicity but it returns a u128
        if let Some(err) = error {
            ValueParam::Text(err)
        } else {
            ValueParam::Null
        },
    ];

    let result = conn.execute(INSERT_COMPLETION, sql_params)?;

    let id_val = result.rows.first().and_then(|r| r.values.first());

    match id_val {
        Some(ValueResult::Integer(id)) => Ok(*id),
        Some(r) => bail!("completion insert returned unexpected type: {:?}", r),
        None => bail!("completion insert did not return id"),
    }
}
