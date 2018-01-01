use rocket_contrib::{Json, Value};
use diesel;
use diesel::prelude::*;
use diesel::QueryResult;
use diesel::result::{Error as DieselError};

use std::error::Error;

use config::database::DbConn;
use tools::error::ProcessError;

use resources::users::models::{Page, User};
use resources::users::serializers::Serialize;
use resources::users::schema;
use self::schema::users;
use self::schema::users::dsl::{users as all_users};

pub struct PageView {
    conn: DbConn,
    page: i32,
    offset: i32
}

impl PageView {
    pub fn new(_conn: DbConn, _page: i32, _offset: i32) -> Self {
        Self {
            conn: _conn,
            page: _page,
            offset: _offset
        }
    }

    ///
    ///
    ///
    ///
    pub fn page(&self) -> Result<Json<Value>, ProcessError> {
        let page = self.page;
        let start = page * self.offset;
        let offset = self.offset as i64;

        let page_query: QueryResult<Vec<User>>
            = all_users.filter(users::id.ge(start))
                .limit(offset)
                .get_results(&*self.conn);

        match page_query {
            Ok(data) => Serialize::page(
                Page::new(data, page, self.offset),
            ),
            Err(error) => Serialize::Error(
                ProcessError::new(
                    404,
                    error.description().to_string()
                )
            ).json()
        }
    }
}
