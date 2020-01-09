use actix_web::{HttpResponse, error};
use serde::{Serialize, Deserialize};
use failure::Fail;

// Error handling

#[derive(Deserialize, Serialize, Debug)]
pub struct R<T> where T: Serialize {
    code: u16,
    message: String,
    data: Option<T>,
}

#[derive(Fail, Debug)]
pub enum ErrorResponse {
    #[fail(display = "An internal error occurred. Connect with maintainer please! ({:?})", reason)]
    InternalError{reason: String},
    #[fail(display = "Not found")]
    NotFound,
    #[fail(display = "login failed wrong user or password")]
    LoginFailed,
    #[fail(display = "Invailed token ``{}'' !", token)]
    InvailedToken{token: String},
    #[fail(display = "No authorization")]
    NoAuthorization,
}

const INTERNALERROR_CODE: u16 = 1;
const NOTFOUND_CODE: u16 = 2;
const LOGINFAILED_CODE: u16 = 3;
const INVAILEDTOKEN_CODE: u16 = 4;
const NOAUTHORIZATION: u16 = 5;

impl error::ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ErrorResponse::NotFound => {
                let r = R::<()>::err(NOTFOUND_CODE, &self.to_string());
                HttpResponse::NotFound().json(r)
            }
            ErrorResponse::LoginFailed => {
                let r = R::<()>::err(LOGINFAILED_CODE, &self.to_string());
                HttpResponse::Unauthorized().json(r)
            }
            ErrorResponse::InvailedToken{..} => {
                let r = R::<()>::err(INVAILEDTOKEN_CODE, &self.to_string());
                HttpResponse::Unauthorized().json(r)
            }
            ErrorResponse::NoAuthorization => {
                let r = R::<()>::err(NOAUTHORIZATION, &self.to_string());
                HttpResponse::Unauthorized().json(r)
            }
            _ => {
                let r = R::<()>::err(INTERNALERROR_CODE, &self.to_string());
                HttpResponse::InternalServerError().json(r)
            }
        }
    }
}

pub fn to_ErrorResponse<E: ToString>(e: E) -> ErrorResponse {
    // panic!("???");
    ErrorResponse::InternalError{reason: e.to_string()}
}

pub type Resp = Result<HttpResponse, ErrorResponse>;

impl<T: Serialize> R<T> {
    pub fn ok(data: T) -> Self {
        R { code: 0, message: "success".to_owned(), data: Some(data) }
    }

    pub fn err(error: u16, message: &str) -> Self {
        R { code: error, message: message.to_owned(), data: None }
    }


    pub fn to_json_result(&self) -> Resp  {
        Ok(HttpResponse::Ok().json(self))
    }
}
