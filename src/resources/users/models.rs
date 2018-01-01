use tools::page::Meta;

use resources::users::schema;
use self::schema::users;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub name: String
}

#[derive(FromForm)]
pub struct PageQuery {
    pub page: i32,
    pub offset: i32
}

#[derive(Serialize, Deserialize)]
pub struct Page {
    pub users: Vec<User>,
    pub meta: Meta
}

impl Page {
    pub fn new(_users: Vec<User>, _page: i32, _offset: i32) -> Self {
        Self {
            users: _users,
            meta: Meta::new(_page, _offset)
        }
    }
}
