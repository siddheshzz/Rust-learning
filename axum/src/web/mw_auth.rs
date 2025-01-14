use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::{Error, Result};
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::{middleware::Next, response::Response};
use lazy_regex::{regex, regex_captures};
use tower_cookies::{Cookie, Cookies};

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

pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    cookies: Cookies,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth ", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

/// Middleware to resolve the context from cookies and attach it to the request.
///
/// This middleware function extracts the authentication token from the cookies,
/// validates it, and constructs a `Ctx` object containing the user ID. The `Ctx`
/// is then stored in the request extensions for further use by downstream handlers.
/// If the token is missing, an `AuthFailNoAuthTokenCookie` error is returned. If
/// the token is invalid, it is removed from the cookies, and an appropriate error
/// is returned.
///
/// # Arguments
///
/// * `_mc` - The model controller state, currently unused.
/// * `cookies` - A `Cookies` instance to access the request cookies.
/// * `ctx` - A `Result<Ctx>` representing the current context, which is checked for errors.
/// * `req` - The incoming `Request` that will be modified with the extracted context.
/// * `next` - The next middleware or handler to run if context resolution is successful.
///
/// # Returns
///
/// Returns a `Result` containing the response. On failure, an error indicating
/// the issue with the authentication token is returned.

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    ctx: Result<Ctx>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver ", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // let result_ctx = match auth_token.ok_or(Error::AuthFailNoAuthTokenCookie).and_then(parse_token){
    //     Ok((user_id, exp, sign)) => Ok(Ctx::new(user_id)),
    //     Err(e) => Err(e)
    // };

    // if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)){
    //     cookie.remove(Cookie::named(AUTH_TOKEN));
    // }

    // req.extensions_mut().insert(result_ctx?);
    // Ok(next.run(req).await)
    // Compute Result<Ctx>.
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            // TODO: Token components validations.
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    // Remove the cookie if something went wrong other than NoAuthTokenCookie.
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx?);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx ", "EXTRACTOR");

        // User cookies extractor.
        // let cookies = parts
        //     .extensions
        //     .get::<Cookies>()
        //     .ok_or(Error::AuthFailNoAuthTokenCookie)?;
        // let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        // //Parse token
        // let (user_id, exp, sign) = auth_token
        //     .ok_or(Error::AuthFailNoAuthTokenCookie)
        //     .and_then(parse_token)?;

        // Ok(Ctx::new(user_id))

        parts
			.extensions
			.get::<Result<Ctx>>()
			.ok_or(Error::AuthFailCtxNotInRequestExt)?
			.clone()
    }
}

/// Parse an authentication token into its three parts: user_id, expiration timestamp, and signature.
///
/// # Errors
///
/// This function will return an error if the token is not in the expected format.
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;
    Ok((user_id, exp.to_string(), sign.to_string()))
}
