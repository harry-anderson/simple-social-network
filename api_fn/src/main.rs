use aws_lambda_events::{
    apigw::{ApiGatewayV2httpRequest as Request, ApiGatewayV2httpResponse as Response},
    encodings::Body,
    http::HeaderMap,
};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing::info;

use crate::error::CustomError;

mod db;
mod error;

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
    info!("event {:?}", event);

    let provider = &event
        .payload
        .path_parameters
        .get("provider")
        .ok_or(CustomError::new("no req param: provider"))?;
    let action = &event
        .payload
        .path_parameters
        .get("action")
        .ok_or(CustomError::new("no req param: acition"))?;

    info!("path params {} {}", provider, action);

    let resp = Response {
        status_code: 200,
        body: Some(Body::Text(String::from("Hello Zordie"))),
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        is_base64_encoded: None,
        cookies: vec![],
    };

    Ok(resp)
}
