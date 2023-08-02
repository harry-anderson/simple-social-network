# Simple social network

A simple social network backend REST API with persistent data.

## Overview

I'm using [AWS serverless application model](https://aws.amazon.com/serverless/sam/) and (1) [rust lambda](https://github.com/awslabs/aws-lambda-rust-runtime) function to implement the API.

![API overview](./static/overview.png)

## Data model

I'm using AWS Dynamodb as a database and [single table design](https://www.alexdebrie.com/posts/dynamodb-single-table) to store and retrieve relational data.

DynamoDB main table

![Main table](./static/single_table_main.png)

DynamoDB global secondary index 1

![GSI 1](./static/single_table_gsi1.png)


## API

### Create User
```bash
curl --request POST \
  --url $BASE_URL/user/create \
  --header 'Content-Type: application/json' \
  --data '{ "user_id": "harry", "password": "pass123" }'
```

### Create Story
```bash
curl --request POST \
  --url $BASE_URL/story/create \
  --header 'Content-Type: application/json' \
  --data '{ "user_id": "harry", "content": "first story" }'
```

### Create Comment
```bash
curl --request POST \
  --url $BASE_URL/comment/create \
  --header 'Content-Type: application/json' \
  --data '{ "user_id": "harry", "story_id": "<STORY_ID>", "content":"A comment" }'
```

### List Stories for User
```bash
curl --request GET \
  --url $BASE_URL/stories/harry
```

### List Comments for Story
```bash
curl --request GET \
  --url $BASE_URL/comments/<STORY_ID>
```

### Delete Comment
```bash
curl --request POST \
  --url $BASE_URL/comment/delete \
  --header 'Content-Type: application/json' \
  --data '{ "comment_id": "<COMMENT_ID>", "story_id": "<STORY_ID>" }'
```

### Delete Story
```bash
curl --request POST \
  --url $BASE_URL/story/delete \
  --header 'Content-Type: application/json' \
  --data '{ "user_id": "harry", "story_id": "<STORY_ID>" }'
```

### Delete User
```bash
curl --request POST \
  --url $BASE_URL/user/delete \
  --header 'Content-Type: application/json' \
  --data '{ "user_id": "harry" }'
```

### Development
```bash
# build and deploy
sam build
sam deploy

#logs
sam logs --stack-name <stackname> --name <FnName>
```
