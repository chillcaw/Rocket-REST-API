extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn connect_db() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("Failed to create pool.")
}

// single connection
// pub fn new_connect_db() -> PgConnection {
//     dotenv().ok();
//
//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }
