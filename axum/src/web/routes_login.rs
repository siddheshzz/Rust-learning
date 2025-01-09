use crate::{web::AUTH_TOKEN, Error};
use serde::Deserialize;
use axum::{routing::post, Json, Router};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use axum::response::IntoResponse;


pub fn routes() -> Router {
    
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies ,payload: Json<LoginPayLoad>) ->Result<impl IntoResponse, Error> {
    println!("->> {:<12} - api_login ","HANDLER");

    //TODO: Real Db/AUTH LOGIC

    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }
    //IMPLEMENT REAL AUTH TOKEN GENERATION/ SIGNATURE
    let mut cookie = Cookie::new(    AUTH_TOKEN,"user-1.exp.sign");
    cookie.set_http_only(true);
	cookie.set_path("/");
	cookies.add(cookie);
    //TO dO Set cookies
    //create the success body

    let body = Json(
        json!({
            "result": {
            "success":true
            }

        })
    );

    Ok::<_, Error>(Json( json!({
        "result": {
        "success":true
        }

    })))

    // Ok(body)
}

#[derive(Debug,Deserialize)]
pub struct LoginPayLoad{
    username: String,
    pwd: String,
}