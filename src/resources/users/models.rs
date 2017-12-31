use resources::users::schema;
use self::schema::users;

#[table_name="users"]
#[derive(Insertable, Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}
