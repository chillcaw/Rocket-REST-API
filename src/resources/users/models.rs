use rocket_contrib::{Json, Value};

use resources::users::schema;
use self::schema::users;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String
}

pub struct Users;

impl Users {
    pub fn json(users: Vec<User>) -> Value {
        Json(json!(users)).into_inner()
    }
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub name: String
}

#[derive(FromForm)]
pub struct PageQuery {
    pub page: Option<i32>,
    pub offset: Option<i32>
}
