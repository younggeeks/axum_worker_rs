use axum::{extract::State, response::Html, routing::get, Router as AxumRouter};
use axum_cloudflare_adapter::{to_axum_request, to_worker_response, wasm_compat, EnvWrapper};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use tower_service::Service;
use worker::*;

#[derive(Debug, Deserialize, Serialize)]
struct GenericResponse {
    status: u16,
    message: String,
}

#[derive(Clone)]
pub struct AxumState {
    pub env_wrapper: EnvWrapper,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let mut router: AxumRouter =
        AxumRouter::new()
            .route("/", get(handle_get))
            .with_state(AxumState {
                env_wrapper: EnvWrapper::new(env),
            });
    let axum_request = to_axum_request(req).await.unwrap();

    let axum_response = router.call(axum_request).await.unwrap();

    let response = to_worker_response(axum_response).await.unwrap();

    Ok(response)
}

#[wasm_compat]
pub async fn handle_get() -> Html<&'static str> {
    Html("<p>Hello there world </p>")
}

pub async fn handle_post(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::from_json(&GenericResponse {
        status: 200,
        message: "You reached a POST route!".to_string(),
    })
}

pub async fn handle_delete(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::from_json(&GenericResponse {
        status: 200,
        message: "You reached a DELETE route!".to_string(),
    })
}
