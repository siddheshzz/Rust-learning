#![allow(unused)]

pub use self::error::{Error,Result};

use std::net::SocketAddr;


use axum::{extract::{Path, Query}, http::StatusCode, middleware::{self, map_response}, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Router};
use model::ModelController;
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::{ServeDir, ServeFile};

mod ctx;
mod error;
mod web;
mod model;

#[derive(Debug,Deserialize)]
struct HelloParams {
    name:Option<String>
}

fn routes_hello() -> Router {
     Router::new()
        .route("/hello",get(handle_hello))
        .route("/hello2/:name",get(handler_hello2))

}

/// Start the Axum server and listen for incoming requests on `127.0.0.1:8080`.
///
/// The server uses the following routes:
/// - `/hello` and `/hello2/:name` for testing purposes.
/// - `/api/login` for logging in.
/// - All other routes will return a 404 status code and a message indicating that the route was not found.
///
/// The server also uses the following middleware layers:
/// - `tower_cookies::CookieManagerLayer` to manage cookies.
/// - `tower_http::services::ServeDir` to serve the static files in the `static` directory.
/// - `axum::middleware::map_response` to map the responses from the routes to the final response sent to the client.
#[tokio::main]
async fn main() ->Result<()> {

    //initialize the model controller
    let mc = ModelController::new()?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
     .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));



    let route_all = Router::new()
        .merge( routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .layer(middleware::from_fn_with_state(mc.clone(),web::mw_auth::mw_ctx_resolver,))
        .fallback(route_static);
    // let addr = SocketAddr::from(([127,0,0,1], 8080));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener,route_all.into_make_service())
        .await
        .unwrap();

        Ok(())
}

async fn handle_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {

    println!("->> {:<12} - handler_hello -{params:?}","HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html("Hello <strong>{name}</strong>")

}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 -{name:?}","HANDLER");

    Html(format!("Hello2 <strong>{name}</strong>"))
}

#[axum::debug_handler]
async fn route_static() -> impl IntoResponse {
    // Router::new().nest_service("/",get_service(ServeDir))
    (StatusCode::NOT_FOUND, "Not Found 'root' - this should never happen")
}


async fn main_response_mapper(res: Response) -> Response { 
    println!("->> {:<12} - main_reponse_mapper ","RES_MAPPER");
    println!();    
    res


}