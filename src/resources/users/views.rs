use rocket_contrib::{Json, Value};
use diesel;
use diesel::prelude::*;

use std::error::Error;

use config::database::DbConn;

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
    pub fn all(&self) -> Json<Value> {
        let all_query
            = all_users.order(users::id.desc())
                .get_results(&*self.conn);

        return Serialize::all(all_query);
    }

    ///
    ///
    ///
    ///
    pub fn find(&self, id: i32) -> Json<Value> {
        let find_query
            = all_users.find(id)
                .get_result(&*self.conn);

        return Serialize::find(find_query);
    }

    ///
    ///
    ///
    ///
    pub fn create(&self, user: NewUser) -> Json<Value> {
        let create_query
            = diesel::insert_into(users::table)
                .values(&user)
                .get_result(&*self.conn);

        return Serialize::create(create_query);
    }

    ///
    ///
    ///
    ///
    pub fn update(&self, id: i32, user: NewUser) -> Json<Value> {
        let update_query = diesel::update(all_users.find(id))
            .set(name.eq(user.name))
            .get_result::<User>(&*self.conn);

        return Serialize::update(update_query);
    }

    ///
    ///
    ///
    ///
    pub fn delete(&self, id: i32) -> Json<Value> {
        let delete_query
            = diesel::delete(all_users.find(id))
                .execute(&*self.conn);

        return Serialize::delete(delete_query);
    }
}
