use rocket_contrib::{Json, Value};
use diesel;
use diesel::prelude::*;

use std::error::Error;

use config::database::DbConn;
use tools::response::{ResSuccess, ResError};

use resources::users::models::{User, NewUser};
use resources::users::serializers::Serialize;
use resources::users::schema;
use self::schema::users;
use self::schema::users::dsl::{users as all_users};
use self::schema::users::columns::{name};

pub struct View {
    conn: DbConn
}

impl View {
    pub fn new(new_conn: DbConn) -> Self {
        Self {conn: new_conn}
    }

    pub fn all(&self) -> Json<Value> {
        let users: Vec<User>
            = all_users.order(users::id.desc())
                .load::<User>(&*self.conn)
                .unwrap();

        return Serialize::Users(users).json();
    }

    pub fn find(&self, id: i32) -> Json<Value> {
        let user: User
            = all_users.find(id)
                .get_result::<User>(&*self.conn)
                .unwrap();

        return Serialize::User(user).json();
    }

    pub fn create(&self, user: NewUser) -> Json<Value> {
        let insert
            = diesel::insert_into(users::table)
                .values(&user)
                .execute(&*self.conn);

        match insert {
            Ok(_) => Serialize::Success(
                ResSuccess::new(201)
            ).json(),

            Err(error) => Serialize::Error(
                ResError::new(
                    422,
                    error.description().to_string()
                )
            ).json()
        }
    }

    pub fn update(&self, id: i32, user: NewUser) -> Json<Value> {
        let updated_user = diesel::update(all_users.find(id))
            .set(name.eq(user.name))
            .get_result::<User>(&*self.conn)
            .unwrap();

        return Serialize::User(updated_user).json();
    }

    pub fn delete(&self, id: i32) -> Json<Value> {
        let delete
            = diesel::delete(all_users.find(id))
                .execute(&*self.conn);

        match delete {
            Ok(_) => Serialize::Success(
                ResSuccess::new(201)
            ).json(),

            Err(error) => Serialize::Error(
                ResError::new(
                    422,
                    error.description().to_string()
                )
            ).json()
        }
    }
}
