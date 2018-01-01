use rocket_contrib::{Json, Value};
use rocket::response::Response;
use rocket::http::{Status, ContentType};

use std::io::Cursor;

use tools::error::ProcessError;

pub struct Build {
    code: u16
}

impl Build {
    pub fn new(data: Result<Json<Value>, ProcessError>, res_code: u16) -> Response<'static> {
        let build = Self { code: res_code };
        build.respond(data)
    }

    pub fn respond(self, data: Result<Json<Value>, ProcessError>) -> Response<'static> {
        match data {
            Ok(data) => self.success_res(data),
            Err(error) => self.error_res(error)
        }
    }

    pub fn success_res(self, data: Json<Value>) -> Response<'static> {
        Response::build()
            .header(ContentType::JSON)
            .status(Status::from_code(self.code).unwrap())
            .sized_body(Cursor::new(data.to_string()))
            .finalize()
    }

    pub fn error_res(self, data: ProcessError) -> Response<'static> {
        Response::build()
            .header(ContentType::JSON)
            .status(Status::from_code(data.code).unwrap())
            .sized_body(Cursor::new(data.to_string()))
            .finalize()
    }
}
