use core::fmt;
use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone,Debug,strum_macros::AsRefStr)]
pub enum Error{
    LoginFail,
    TicketDeleteFailIdNotFound{id: u64},

    // Auth error
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{self:?}")
    }
}

impl std::error::Error for Error {

}


impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}","INTRO_RES");
        //place holder axum response.

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        //Insert the error into the response

        response.extensions_mut().insert(self);
        response
    }



}


impl Error{
    pub fn client_status_and_error(&self) -> (StatusCode,ClientError){
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

			// -- Auth.
			Self::AuthFailNoAuthTokenCookie
			| Self::AuthFailTokenWrongFormat
			| Self::AuthFailCtxNotInRequestExt => {
				(StatusCode::FORBIDDEN, ClientError::NO_AUTH)
			}

            Self::TicketDeleteFailIdNotFound { .. } => {(
                StatusCode::BAD_REQUEST,
                ClientError::INVALID_PARAMS,)}


            _ =>(
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR
            ),
            
        }
    }
}

#[derive(Debug,strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError{
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

