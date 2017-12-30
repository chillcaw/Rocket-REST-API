use rocket_contrib::{Json, Value};

use resources::users;
use self::users::models::{User, Users};

#[derive(Serialize, Deserialize)]
pub enum Serial {
    User(User),
    Users(Users)
}

impl Serial {
    pub fn json(&self) -> Json<Value> {
        match *self {
            Serial::User(ref User)
                => Json(json!(User.clone())),
            Serial::Users(ref Users)
                => Json(json!(Users.clone()))
        }
    }
}
