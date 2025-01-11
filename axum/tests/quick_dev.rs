#![allow(unused)]

use anyhow::Result;
use axum::http::request;
use serde_json::json;

    /// Quick smoke test for local dev environment.
    ///
    /// This test is disabled by default, as it's not meant to be run on CI
    /// (in fact, it will fail on CI as it relies on the server running on localhost).
    ///
    /// It is meant to be run locally to quickly test the server is running as expected.
    ///

#[tokio::test]
async fn quick_dev() -> Result<()> {

    let hc =  httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello2/Mike").await?.print().await?;
    //for static fallback testing
    hc.do_get("/src/main.rs").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }
        )
    );
    req_login.await?.print().await?;


    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket 1"
        }
        )
    );
    req_create_ticket.await?.print().await?;
    hc.do_get("/api/tickets").await?.print().await?;
  
    Ok(())

}