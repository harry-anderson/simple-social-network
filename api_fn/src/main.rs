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
    _db_client: &db::DbClient,
) -> Result<Response, Error> {
    info!("event {:?}", event);
    let method = &event.payload.request_context.http.method;

    match *method {
        Method::GET => {
            let entity = path_param(&event, "entity").await?;
            let id = path_param(&event, "id").await?;

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
            let entity = path_param(&event, "entity").await?;
            let action = path_param(&event, "action").await?;
            let body = &event.payload.body.unwrap_or(String::from("None"));
            //
            let res = format!("path params {} {} {:?}", entity, action, body);
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

async fn path_param(event: &LambdaEvent<Request>, name: &str, ) -> Result<String, Error> {
    let param = event
        .payload
        .path_parameters
        .get(name)
        .ok_or(CustomError::new(&format!("no path param: {name}")))?;

    Ok(param.to_string())
}

