-- list apps
SELECT a.*,
  GROUP_CONCAT(m.role || ':' || m.content) AS messages
FROM apps AS a
  LEFT JOIN messages AS m ON a.id = m.app_id
GROUP BY a.id;

-- get app by name
SELECT a.*,
  GROUP_CONCAT(m.role || ':' || m.content) AS messages
FROM apps AS a
  LEFT JOIN messages AS m ON a.id = m.app_id
WHERE a.name = ?;

-- upsert app
INSERT
  OR REPLACE INTO apps (
    id,
    created_at,
    updated_at,
    name,
    description,
    model_name,
    model_max_tokens,
    model_temperature,
    model_repeat_penalty,
    model_repeat_penalty_last_n_tokens,
    model_top_k,
    model_top_p
  )
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- delete app
DELETE FROM messages
WHERE app_id = ?;
DELETE FROM apps
WHERE id = ?;
