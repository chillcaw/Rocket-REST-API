use rocket_contrib::{Json, Value};
use diesel;
use diesel::prelude::*;

use config::database::DbConn;
use resources::users::models::User;
use resources::users::serializers::Serial;

use resources::users::schema;
use self::schema::users;
use self::schema::users::dsl::{users as all_users};

pub struct View {
    conn: DbConn
}

impl View {
    pub fn new(new_conn: DbConn) -> Self {
        Self {conn: new_conn}
    }

    pub fn all(&self) -> Json<Value> {
        let data = all_users.order(users::id.desc()).load::<User>(&*self.conn).unwrap();
        return Serial::Users(data).json();
    }

    pub fn find(&self, id: i32) -> Json<Value> {
        let data = all_users.find(id).get_result::<User>(&*self.conn).unwrap();
        return Serial::User(data).json();
    }

    pub fn create(&self, user: User) -> Json<Value> {
        diesel::insert_into(users::table).values(&user).execute(&*self.conn).is_ok();
        return Serial::User(user).json();
    }

    pub fn update(&self, id: i32, user: User) -> Json<Value> {
        diesel::delete(all_users.find(id)).execute(&*self.conn);
        diesel::insert_into(users::table).values(&user).execute(&*self.conn).is_ok();
        return Serial::User(user).json();
    }

    pub fn delete(&self, id: i32) -> Json<Value> {
        diesel::delete(all_users.find(id)).execute(&*self.conn);
        return Serial::User(proto_user()).json();
    }
}
