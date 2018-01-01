use rocket_contrib::{Json, Value};
use diesel::QueryResult;
use diesel::result::{Error as DieselError};

use std::error::Error;

use resources::users;
use self::users::models::User;
use tools::response::{ProcessError, ProcessSuccess};

pub enum Serialize {
    User(User),
    Users(Vec<User>),
    Delete(ProcessSuccess),

    Error(ProcessError),
}

impl Serialize {
    pub fn json(self) -> Result<Json<Value>, ProcessError> {
        match self {
            Serialize::User(ref user)
                => Ok(Json(json!(user.clone()))),
            Serialize::Users(ref users)
                => Ok(Json(json!(users.clone()))),
            Serialize::Delete(ref delete)
                => Ok(Json(json!(delete.clone()))),
            Serialize::Error(error)
                => Err(error),
        }
    }

    pub fn all(query: QueryResult<Vec<User>>) -> Result<Json<Value>, ProcessError> {
        match query {
            Ok(users) => self::Serialize::Users(
                users
            ).json(),

            Err(error) => self::Serialize::Error(
                ProcessError::new(
                    404,
                    error.description().to_string()
                )
            ).json()
        }
    }

    pub fn find(query: QueryResult<User>) -> Result<Json<Value>, ProcessError> {
        match query {
            Ok(user) => self::Serialize::User(
                user
            ).json(),

            Err(error) => self::Serialize::Error(
                ProcessError::new(
                    404,
                    error.description().to_string()
                )
            ).json()
        }
    }

    pub fn update(query: QueryResult<User>) -> Result<Json<Value>, ProcessError> {
        match query {
            Ok(user) => self::Serialize::User(
                user
            ).json(),

            Err(error) => self::Serialize::Error(
                ProcessError::new(
                    self::Serialize::update_error(&error),
                    error.description().to_string()
                )
            ).json()
        }
    }

    pub fn update_error(error: &DieselError) -> u16 {
        if error == &DieselError::NotFound {
            return 404;
        }
        return 422;
    }

    pub fn create(query: QueryResult<User>) -> Result<Json<Value>, ProcessError> {
        match query {
            Ok(user) => self::Serialize::User(
                user
            ).json(),

            Err(error) => self::Serialize::Error(
                ProcessError::new(
                    422,
                    error.description().to_string()
                )
            ).json()
        }
    }

    pub fn delete(query: QueryResult<usize>) -> Result<Json<Value>, ProcessError> {
        match query {
            Ok(_) => self::Serialize::Delete(
                ProcessSuccess::new(200)
            ).json(),

            Err(error) => self::Serialize::Error(
                ProcessError::new(
                    404,
                    error.description().to_string()
                )
            ).json()
        }
    }
}
