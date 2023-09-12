#!/usr/bin/env bash

set -euo pipefail

echo "DELETE"
curl -i -X DELETE http://127.0.0.1:3000/api/app/my-first-prompt-app
echo -e "\nexit: $?\n-------------------"

echo "POST"
curl -i -X POST http://127.0.0.1:3000/api/app \
-H "Content-Type: application/json" \
-d @- << EOF
{
  "name": "my-first-prompt-app",
  "description": "This is my first prompt app!",
  "system_prompt": "You are a helpful, respectful and honest assistant. Always answer as helpfully as possible, while being safe.  Your answers should not include any harmful, unethical, racist, sexist, toxic, dangerous, or illegal content. Please ensure that your responses are socially unbiased and positive in nature.\n\nIf a question does not make any sense, or is not factually coherent, explain why instead of answering something not correct. If you don't know the answer to a question, please don't share false information.",
  "model": {
    "name": "llama2-chat",
    "max_tokens": 75,
    "temperature": 0.0,
    "repeat_penalty": 1.1,
    "repeat_penalty_last_n_tokens": 64,
    "top_k": 0,
    "top_p": 1.0
  }
}
EOF
sleep 1
echo -e "\nexit: $?\n-------------------"

echo "GET(all)"
curl -i http://127.0.0.1:3000/api/app
sleep 1
echo -e "\nexit: $?\n-------------------"

echo "GET(all)?offset=0&limit=1"
curl -i http://127.0.0.1:3000/api/app?offset=0&limit=1
sleep 1
echo -e "\nexit: $?\n-------------------"

echo "GET(name)"
curl -i http://127.0.0.1:3000/api/app/my-first-prompt-app
sleep 1
echo -e "\nexit: $?\n-------------------"

echo "PUT"
curl -i -X PUT http://127.0.0.1:3000/api/app/my-first-prompt-app \
-H "Content-Type: application/json" \
-d @- << EOF
{
  "name": "my-first-prompt-app",
  "description": "This is my first prompt app!",
  "system_prompt": "You are a helpful, respectful and honest assistant. Always answer as helpfully as possible, while being safe.  Your answers should not include any harmful, unethical, racist, sexist, toxic, dangerous, or illegal content. Please ensure that your responses are socially unbiased and positive in nature.\n\nIf a question does not make any sense, or is not factually coherent, explain why instead of answering something not correct. If you don't know the answer to a question, please don't share false information.",
  "model": {
    "name": "llama2-chat",
    "max_tokens": 75,
    "temperature": 0.0,
    "repeat_penalty": 1.1,
    "repeat_penalty_last_n_tokens": 64,
    "top_k": 0,
    "top_p": 1.0
  }
}
EOF
sleep 1
echo -e "\nexit: $?\n-------------------"
