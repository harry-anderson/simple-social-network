use std::collections::HashMap;

use aws_lambda_events::{
    apigw::{ApiGatewayV2httpRequest as Request, ApiGatewayV2httpResponse as Response},
    encodings::Body,
    http::HeaderMap,
};
use aws_sdk_dynamodb::types::AttributeValue;
use http::Method;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use model::{Comment, Entity, Story, User};
use serde_json::{json, to_string};

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
    // info!("event {:?}", event);
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
                                (String::from(":pk"), AttributeValue::S(format!("story#{id}"))),
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
                    let Ok(request) = serde_json::from_str::<User>(body) else {
                        return Ok(response(400, Some(Body::Text(String::from("malformed request")))))
                    };
                    let ent: Entity = request.into();
                    let res = serde_json::to_string(&ent)?;
                    let _db_res = db_client.put(ent).await?;

                    Ok(response(200, Some(Body::Text(res))))
                }
                ("story", "create") => {
                    let Ok(request) = serde_json::from_str::<Story>(body) else {
                        return Ok(response(400, Some(Body::Text(String::from("malformed request")))))
                    };
                    let ent: Entity = request.into();
                    let res = serde_json::to_string(&ent)?;
                    let _db_res = db_client.put(ent).await?;

                    Ok(response(200, Some(Body::Text(res))))
                }
                ("comment", "create") => {
                    let Ok(request) = serde_json::from_str::<Comment>(body) else {
                        return Ok(response(400, Some(Body::Text(String::from("malformed request")))))
                    };
                    let ent: Entity = request.into();
                    let res = serde_json::to_string(&ent)?;
                    let _db_res = db_client.put(ent).await?;

                    Ok(response(200, Some(Body::Text(res))))
                }
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
