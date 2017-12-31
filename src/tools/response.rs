#[derive(Serialize, Deserialize)]
pub struct ResError {
    pub code: i32,
    pub message: String
}

impl ResError {
    pub fn new(status_code: i32, error_message: String) -> Self {
        Self {
            code: status_code,
            message: error_message
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ResSuccess {
    pub code: i32,
    pub message: String
}

impl ResSuccess {
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
