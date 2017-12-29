extern crate rocket;

pub mod database;

pub fn init_app() -> rocket::Rocket {
    return rocket::ignite()
        .manage(database::connect_db());
}
