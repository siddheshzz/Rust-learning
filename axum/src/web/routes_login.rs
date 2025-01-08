use crate::{Error,Result};
use serde::Deserialize;
use axum::{routing::post, Json, Router};
use serde_json::{json, Value};


pub fn routes() -> Router {
    
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayLoad>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login ","HANDLER");

    //TODO: Real Db/AUTH LOGIC

    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    //TO dO Set cookies
    //create the success body

    let body = Json(
        json!({
            "result": {
            "success":true
            }

        })
    );

    Ok(body)
}

#[derive(Debug,Deserialize)]
pub struct LoginPayLoad{
    username: String,
    pwd: String,
}