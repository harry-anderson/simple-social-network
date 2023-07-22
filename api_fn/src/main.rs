use aws_lambda_events::{
    apigw::{ApiGatewayV2httpRequest as Request, ApiGatewayV2httpResponse as Response},
    encodings::Body,
    http::HeaderMap,
};
use http::Method;
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
    let method = &event.payload.request_context.http.method;

    match *method {
        Method::GET => {
            let entity = &event
                .payload
                .path_parameters
                .get("entity")
                .ok_or(CustomError::new("no path param: entity"))?;
            let id = &event
                .payload
                .path_parameters
                .get("id")
                .ok_or(CustomError::new("no path param: id"))?;

            let res = format!("path params {} {}", entity, id);

            let resp = Response {
                status_code: 200,
                body: Some(Body::Text(res)),
                headers: HeaderMap::new(),
                multi_value_headers: HeaderMap::new(),
                is_base64_encoded: None,
                cookies: vec![],
            };
            Ok(resp)
        }
        Method::POST => {
            let entity = &event
                .payload
                .path_parameters
                .get("entity")
                .ok_or(CustomError::new("no path param: entity"))?;
            let action = &event
                .payload
                .path_parameters
                .get("action")
                .ok_or(CustomError::new("no path param: action"))?;
            //
            let res = format!("path params {} {}", entity, action);
            Ok(Response {
                status_code: 200,
                body: Some(Body::Text(res)),
                headers: HeaderMap::new(),
                multi_value_headers: HeaderMap::new(),
                is_base64_encoded: None,
                cookies: vec![],
            })
        }
        _ => Ok(Response {
            status_code: 404,
            body: None,
            headers: HeaderMap::new(),
            multi_value_headers: HeaderMap::new(),
            is_base64_encoded: None,
            cookies: vec![],
        }),
    }
}
