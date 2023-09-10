#!/usr/bin/env bash

echo "POST"
curl -i -X POST http://127.0.0.1:3000/api/app \
-H "Content-Type: application/json" \
-d @- << EOF
{
  "id": 1,
  "created_at": "2021-08-31T18:00:00.000Z",
  "updated_at": "2021-08-31T18:00:00.000Z",
  "name": "my-first-prompt-app",
  "description": "This is my first prompt app!",
  "messages": [
    {
      "role": "system",
      "content": "Welcome to your new prompt app!"
    },
    {
      "role": "user",
      "content": "Hello, world!"
    }
  ],
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
echo -e "\nexit: $?\n-------------------"

echo "GET(all)"
curl -i http://127.0.0.1:3000/api/app
echo -e "\nexit: $?\n-------------------"

echo "GET(all)?offset=0&limit=1"
curl -i http://127.0.0.1:3000/api/app?offset=0&limit=1
echo -e "\nexit: $?\n-------------------"

echo "GET(name)"
curl -i http://127.0.0.1:3000/api/app/my-first-prompt-app
echo -e "\nexit: $?\n-------------------"

echo "PUT"
curl -i -X PUT http://127.0.0.1:3000/api/app/my-first-prompt-app \
-H "Content-Type: application/json" \
-d @- << EOF
{
  "id": 1,
  "created_at": "2021-08-31T18:00:00.000Z",
  "updated_at": "2021-08-31T18:00:00.000Z",
  "name": "my-first-prompt-app",
  "description": "This is my first prompt app!",
  "messages": [
    {
      "role": "system",
      "content": "Welcome to your new prompt app!"
    },
    {
      "role": "user",
      "content": "Hello, world!"
    }
  ],
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
echo -e "\nexit: $?\n-------------------"

echo "DELETE"
curl -i -X DELETE http://127.0.0.1:3000/api/app/my-first-prompt-app
echo -e "\nexit: $?\n-------------------"
