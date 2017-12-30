use std::fmt::Debug;

use rocket_contrib::{Json, Value};
use serde_derive;

use resources::users;
use self::users::models::{User, Users};
use self::users::serializers::Serial;

fn proto_user() -> User {
    return User {
        id: 1,
        name: String::from("Calum")
    }
}

fn proto_users() -> Users {
    return Users {
        users: vec![
            User {
                id: 2,
                name: String::from("Calum")
            },
            User {
                id: 3,
                name: String::from("Calum2")
            }
        ]
    }
}

pub fn all() -> Json<Value> {
    return Serial::Users(
        proto_users()
    ).json();
}

pub fn find(id: usize) -> Json<Value> {
    return Serial::User(
        proto_user()
    ).json();
}

pub fn create(user: User) -> Json<Value> {
    return Serial::User(
        proto_user()
    ).json();
}

pub fn update(id: usize, user: User) -> Json<Value> {
    return Serial::User(
        proto_user()
    ).json();
}

pub fn delete(id: usize) -> Json<Value> {
    return Serial::User(
        proto_user()
    ).json();
}
