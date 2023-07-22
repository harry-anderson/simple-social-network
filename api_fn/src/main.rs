use std::collections::HashMap;

use aws_lambda_events::{
    apigw::{ApiGatewayV2httpRequest as Request, ApiGatewayV2httpResponse as Response},
    encodings::Body,
    http::HeaderMap,
};
use aws_sdk_dynamodb::types::AttributeValue;
use http::Method;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use model::{CreateCommentInput, CreateStoryInput, CreateUserInput, Entity};
use serde_json::{json, to_string};
use uuid::Uuid;

use crate::error::CustomError;

mod db;
mod error;
mod model;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let db = db::DbClient::new("social-network").await;
    let db_ref = &db;

    let func = service_fn(move |event| async move { function_handler(event, db_ref).await });

    run(func).await
}

async fn function_handler(
    event: LambdaEvent<Request>,
    db_client: &db::DbClient,
) -> Result<Response, Error> {
    let method = &event.payload.request_context.http.method;
    match *method {
        Method::GET => {
            let entity = path_param(&event, "entity").await?;
            let id = path_param(&event, "id").await?;

            match entity.as_str() {
                "stories" => {
                    let res = db_client
                        .query::<Entity>(
                            "#pk = :pk and begins_with(#sk, :sk)",
                            HashMap::from([
                                (String::from("#pk"), String::from("PK")),
                                (String::from("#sk"), String::from("SK")),
                            ]),
                            HashMap::from([
                                (String::from(":pk"), AttributeValue::S(format!("user#{id}"))),
                                (
                                    String::from(":sk"),
                                    AttributeValue::S(String::from("story#")),
                                ),
                            ]),
                            None,
                        )
                        .await?;
                    let json = to_string(&res)?;
                    Ok(response(200, Some(Body::Text(json))))
                }
                "comments" => {
                    let res = db_client
                        .query::<Entity>(
                            "#pk = :pk and begins_with(#sk, :sk)",
                            HashMap::from([
                                (String::from("#pk"), String::from("SK")),
                                (String::from("#sk"), String::from("PK")),
                            ]),
                            HashMap::from([
                                (
                                    String::from(":pk"),
                                    AttributeValue::S(format!("story#{id}")),
                                ),
                                (
                                    String::from(":sk"),
                                    AttributeValue::S(String::from("comment#")),
                                ),
                            ]),
                            Some(String::from("GSI1")),
                        )
                        .await?;

                    let json = to_string(&res)?;
                    Ok(response(200, Some(Body::Text(json))))
                }
                _ => Ok(response(
                    400,
                    Some(Body::Text(String::from("invalid request"))),
                )),
            }
        }
        Method::POST => {
            let entity = path_param(&event, "entity").await?;
            let action = path_param(&event, "action").await?;
            let Some(body) = &event.payload.body else {
                return Ok(response(400, Some(Body::Text(String::from("no request body")))))
            };

            match (entity.as_str(), action.as_str()) {
                ("user", "create") => {
                    let Ok(input) = serde_json::from_str::<CreateUserInput>(body) else {
                        return Ok(response(400, Some(Body::Text(String::from("malformed request")))))
                    };
                    let user_id = input.user_id.clone();
                    let ent = Entity::User {
                        pk: format!("user#{user_id}"),
                        sk: format!("user#{user_id}"),
                        password: input.password,
                    };
                    let _ = db_client.put(ent).await?;
                    let res = json!({ "user_id": user_id }).to_string();
                    Ok(response(200, Some(Body::Text(res))))
                }
                ("story", "create") => {
                    let Ok(input) = serde_json::from_str::<CreateStoryInput>(body) else {
                        return Ok(response(400, Some(Body::Text(String::from("malformed request")))))
                    };
                    let story_id = Uuid::new_v4();
                    let ent = Entity::Story {
                        pk: format!("user#{}", input.user_id),
                        sk: format!("story#{story_id}"),
                        story_text: input.content,
                    };
                    let _ = db_client.put(ent).await?;
                    let res = json!({ "story_id": story_id.to_string() }).to_string();
                    Ok(response(200, Some(Body::Text(res))))
                }
                ("comment", "create") => {
                    let Ok(input) = serde_json::from_str::<CreateCommentInput>(body) else {
                        return Ok(response(400, Some(Body::Text(String::from("malformed request")))))
                    };
                    let comment_id = Uuid::new_v4();
                    let ent = Entity::Comment {
                        pk: format!("comment#{}", comment_id),
                        sk: format!("story#{}", input.story_id),
                        user_id: input.user_id,
                        comment_text: input.content,
                    };
                    let _ = db_client.put(ent).await?;
                    let res = json!({ "comment_id": comment_id.to_string() }).to_string();
                    Ok(response(200, Some(Body::Text(res))))
                }
                //TODO deletes
                (_, _) => Ok(response(
                    400,
                    Some(Body::Text(String::from("invalid request"))),
                )),
            }
        }
        _ => Ok(response(400, None)),
    }
}

async fn path_param(event: &LambdaEvent<Request>, name: &str) -> Result<String, Error> {
    let param = event
        .payload
        .path_parameters
        .get(name)
        .ok_or(CustomError::new(&format!("no path param: {name}")))?;

    Ok(param.to_string())
}

fn response(status_code: i64, body: Option<Body>) -> Response {
    Response {
        status_code,
        body,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        is_base64_encoded: None,
        cookies: vec![],
    }
}
