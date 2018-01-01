use rocket_contrib::{Json, Value};
use rocket::response::Response;
use rocket::http::{Status, ContentType};

use std::io::Cursor;
use std::error::Error;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessError {
    pub code: u16,
    pub message: String
}

impl ProcessError {
    pub fn new(status_code: u16, error_message: String) -> Self {
        Self {
            code: status_code,
            message: error_message
        }
    }
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let written = Json(json!(
            {
                "code": &self.code,
                "message": &self.message
            }
        )).to_string();

        write!(f, "{}", written)
    }
}

impl Error for ProcessError {
    fn description(&self) -> &str {
        &self.message
    }

}

#[derive(Serialize, Deserialize)]
pub struct ProcessSuccess {
    pub code: u16,
    pub message: String
}

impl ProcessSuccess {
    pub fn new(status_code: u16) -> Self {
        Self {
            code: status_code,
            message: Self::message(status_code),
        }
    }

    fn message(status_code: u16) -> String {
        let message = match status_code {
            200 => "Ok",
            201 => "Created",
            202 => "Accepted",
            _ => "Bad code"
        };

        return String::from(message);
    }
}

pub struct Build;

impl Build {
    pub fn new(data: Result<Json<Value>, ProcessError>) -> Response<'static> {
        Self::respond(data)
    }

    pub fn respond(data: Result<Json<Value>, ProcessError>) -> Response<'static> {
        match data {
            Ok(data) => Self::success_res(data),
            Err(error) => Self::error_res(error)
        }
    }

    pub fn success_res(data: Json<Value>) -> Response<'static> {
        Response::build()
            .header(ContentType::JSON)
            .status(Status::from_code(200).unwrap())
            .sized_body(Cursor::new(data.to_string()))
            .finalize()
    }

    pub fn error_res(data: ProcessError) -> Response<'static> {
        Response::build()
            .header(ContentType::JSON)
            .status(Status::from_code(data.code).unwrap())
            .sized_body(Cursor::new(data.to_string()))
            .finalize()
    }
}
