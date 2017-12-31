#[derive(Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub message: String
}

impl Error {
    pub fn new(status_code: i32) -> Self {
        Self {
            code: status_code,
            message: Self::message(status_code)
        }
    }

    fn message(status_code: i32) -> String {
        let message = match status_code {
            422 => "Unprocessable Entity",
            _ => "Bad code"
        };

        return String::from(message);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Success {
    pub code: i32,
    pub message: String
}

impl Success {
    pub fn new(status_code: i32) -> Self {
        Self {
            code: status_code,
            message: Self::message(status_code)
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
