use axum::http::Request;
use axum::{ middleware::Next, response::Response};
use tower_cookies::Cookies;
use crate::{Error, Result};
use axum::body::Body;

use crate::web::AUTH_TOKEN;

/// Middleware to require authentication by checking for an auth token cookie.
///
/// This middleware function ensures that an incoming request contains a valid
/// authentication token cookie. If the auth token is present, the request is
/// forwarded to the next handler in the chain. If the auth token is missing, 
/// an `AuthFailNoAuthTokenCookie` error is returned.
///
/// # Arguments
///
/// * `cookies` - A `Cookies` instance to access the request cookies.
/// * `req` - The incoming `Request` to be processed.
/// * `next` - The next middleware or handler to run if authentication is successful.
///
/// # Returns
///
/// Returns a `Result` containing the response. On failure, an error indicating
/// the absence of the auth token cookie is returned.

pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next:Next) -> Result<Response>{

    println!("->> {:<12} - mw_require_auth ", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());


    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie )?;
    Ok(next.run(req).await)
}


fn parse_token(token:String) ->Result<(u64, String, String)>{
    todo!()
}