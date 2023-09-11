SELECT * FROM apps;
SELECT * FROM messages;

DELETE FROM messages;
DELETE FROM apps;

INSERT INTO apps (
    'id',
    'created_at',
    'updated_at',
    'name',
    'description',
    'model_name',
    'model_max_tokens',
    'model_temperature',
    'model_repeat_penalty',
    'model_repeat_penalty_last_n_tokens',
    'model_top_k',
    'model_top_p'
) VALUES (
    1,
    '2021-08-31T18:00:00.000Z',
    '2021-08-31T18:00:00.000Z',
    'my-first-prompt-app',
    'This is my first prompt app!',
    'llama2-chat',
    75,
    0.0,
    1.1,
    64,
    40,
    0.9
);

INSERT INTO messages (
    'id',
    'app_id',
    'role',
    'content'
) VALUES (
    1,
    1,
    'system',
    'You are a helpful, respectful and honest assistant. Always answer as helpfully as possible, while being safe.  Your answers should not include any harmful, unethical, racist, sexist, toxic, dangerous, or illegal content. Please ensure that your responses are socially unbiased and positive in nature.\n\nIf a question does not make any sense, or is not factually coherent, explain why instead of answering something not correct. If you don''t know the answer to a question, please don''t share false information.'
);

INSERT INTO messages (
    'id',
    'app_id',
    'role',
    'content'
) VALUES (
    2,
    1,
    'user',
    'What is the meaning of life?'
);
