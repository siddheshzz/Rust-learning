#![allow(unused)]
// /... .. -.. -.. .-.- .... . ... ....
pub use self::error::{Error, Result};

use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    http::{uri, Method, StatusCode, Uri},
    middleware::{self, map_response},
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Json, Router,
};
use ctx::Ctx;
use log::log_request;
use model::ModelController;
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::{ServeDir, ServeFile};
use uuid::Uuid;

mod ctx;
mod error;
mod log;
mod model;
mod web;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handle_hello))
        .route("/hello2/:name", get(handler_hello2))
}

/// Main entry point of the application.
///
/// It sets up a tokio listener on localhost:8080 and creates a router with the following routes:
///
/// *   `/hello` - a simple hello world route
/// *   `/hello2/:name` - a route that takes a name as a parameter
/// *   `/api` - a protected route that requires authentication
/// *   `/api/tickets` - a route to list all tickets
/// *   `/api/tickets/:id` - a route to delete a ticket
/// *   `/api/login` - a route to login and get an authentication token
/// *   `/*` - a fallback route that returns a 404 error
///
/// The authentication token is stored in a cookie.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //initialize the model controller
    let mc = ModelController::new()?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let route_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new());
    // .fallback(route_static);
    // let addr = SocketAddr::from(([127,0,0,1], 8080));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind TCP listner");
    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, route_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn handle_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello -{params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html("Hello <strong>{name}</strong>")
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 -{name:?}", "HANDLER");

    Html(format!("Hello2 <strong>{name}</strong>"))
}

#[axum::debug_handler]
async fn route_static() -> impl IntoResponse {
    // Router::new().nest_service("/",get_service(ServeDir))
    (
        StatusCode::NOT_FOUND,
        "Not Found 'root' - this should never happen",
    )
}

async fn main_response_mapper(
    ctx:Option<Ctx>,
    uri:Uri,
    req_method: Method,
    res: Response
) -> Response {
    println!("->> {:<12} - main_reponse_mapper ", "RES_MAPPER");

    let uuid = Uuid::new_v4();

    //get the eventual response error
    let service_error = res.extensions().get::<Error>();

    let client_status_error = service_error.map(|se| se.client_status_and_error());

    //if client error build the new response
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error":{
                    "type":client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            println!("->>  - client_error_body - {client_error_body}",);
            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });
    // Build and log the server log line.
    let client_error = client_status_error.unzip().1;

    let _ = log_request(
        uuid,
        req_method,
        uri,
        ctx,
        service_error,
        client_error
    ).await;
    // TODO: Need to hander if log_request fail (but should not fail request)

    // println!("->> server log line - {uuid} - Error: {service_error:?}",);    

    println!();
    error_response.unwrap_or(res)
}
