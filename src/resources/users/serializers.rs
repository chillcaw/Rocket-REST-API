use rocket_contrib::{Json, Value};

use resources::users;
use self::users::models::{User, Users};

pub enum Serial {
    User(User),
    Users(Users)
}

impl Serial {
    pub fn json(&self) -> Json<Value> {
        match *self {
            Serial::User(ref user)
                => Json(json!(user.clone())),
            Serial::Users(ref users)
                => Json(json!(users.clone()))
        }
    }
}
