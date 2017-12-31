use rocket_contrib::{Json, Value};

use resources::users;
use self::users::models::User;
use tools::response::{ResSuccess, ResError};

pub enum Serialize {
    User(User),
    Users(Vec<User>),
    Error(ResError),
    Success(ResSuccess)
}

impl Serialize {
    pub fn json(&self) -> Json<Value> {
        match *self {
            Serialize::User(ref user)
                => Json(json!(user.clone())),
            Serialize::Users(ref users)
                => Json(json!(users.clone())),
            Serialize::Error(ref error)
                => Json(json!(error.clone())),
            Serialize::Success(ref success)
                => Json(json!(success.clone()))
        }
    }
}
