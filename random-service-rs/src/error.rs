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
    #[fail(display = "An internal error occurred. Connect with maintainer please!")]
    InternalError,
    #[fail(display = "Not found")]
    NotFound,
}

const INTERNALERROR_CODE: u16 = 1;
const NOTFOUND_CODE: u16 = 2;

impl error::ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ErrorResponse::NotFound => {
                let r = R::<()>::err(NOTFOUND_CODE, &self.to_string());
                HttpResponse::NotFound().json(r)
            }
            _ => {
                let r = R::<()>::err(INTERNALERROR_CODE, &self.to_string());
                HttpResponse::InternalServerError().json(r)
            }
        }
    }

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
