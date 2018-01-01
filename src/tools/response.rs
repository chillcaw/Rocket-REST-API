#[derive(Serialize, Deserialize)]
pub struct ProcessError {
    pub code: i32,
    pub message: String
}

impl ProcessError {
    pub fn new(status_code: i32, error_message: String) -> Self {
        Self {
            code: status_code,
            message: error_message
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProcessSuccess {
    pub code: i32,
    pub message: String
}

impl ProcessSuccess {
    pub fn new(status_code: i32) -> Self {
        Self {
            code: status_code,
            message: Self::message(status_code),
        }
    }

    fn message(status_code: i32) -> String {
        let message = match status_code {
            200 => "Ok",
            201 => "Created",
            202 => "Accepted",
            _ => "Bad code"
        };

        return String::from(message);
    }
}
