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
    pub name: String,
}
