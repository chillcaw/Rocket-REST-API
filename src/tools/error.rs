use rocket_contrib::Json;

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
