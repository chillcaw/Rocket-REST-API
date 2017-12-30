use r2d2;
use std::env;

use dotenv::dotenv;
use r2d2_diesel::ConnectionManager;
use diesel::pg::PgConnection;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_db() -> Pool {
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
