use rocket_contrib::{Json, Value};
use diesel;
use diesel::prelude::*;
use diesel::QueryResult;

use config::database::DbConn;
use tools::response::ProcessError;

use resources::users::models::{User, NewUser};
use resources::users::serializers::Serialize;
use resources::users::schema;
use self::schema::users;
use self::schema::users::dsl::{users as all_users};
use self::schema::users::columns::name;

pub struct View {
    conn: DbConn
}

impl View {
    ///
    ///
    ///
    ///
    pub fn new(new_conn: DbConn) -> Self {
        Self {conn: new_conn}
    }

    ///
    ///
    ///
    ///
    pub fn all(&self) -> Result<Json<Value>, ProcessError> {
        let all_query: QueryResult<Vec<User>>
            = all_users.order(users::id.desc())
                .get_results(&*self.conn);

        return Serialize::all(all_query);
    }

    ///
    ///
    ///
    ///
    pub fn find(&self, id: i32) -> Result<Json<Value>, ProcessError> {
        let find_query: QueryResult<User>
            = all_users.find(id)
                .get_result(&*self.conn);

        return Serialize::find(find_query);
    }

    ///
    ///
    ///
    ///
    pub fn create(&self, user: NewUser) -> Result<Json<Value>, ProcessError> {
        let create_query: QueryResult<User>
            = diesel::insert_into(users::table)
                .values(&user)
                .get_result(&*self.conn);

        return Serialize::create(create_query);
    }

    ///
    ///
    ///
    ///
    pub fn update(&self, id: i32, user: NewUser) -> Result<Json<Value>, ProcessError> {
        let update_query: QueryResult<User>
            = diesel::update(all_users.find(id))
                .set(name.eq(user.name))
                .get_result::<User>(&*self.conn);

        return Serialize::update(update_query);
    }

    ///
    ///
    ///
    ///
    pub fn delete(&self, id: i32) -> Result<Json<Value>, ProcessError> {
        let delete_query: QueryResult<usize>
            = diesel::delete(all_users.find(id))
                .execute(&*self.conn);

        return Serialize::delete(delete_query);
    }
}
