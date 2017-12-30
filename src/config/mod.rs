use rocket;

pub mod database;

pub fn init_app() -> rocket::Rocket {
    return rocket::ignite()
        .manage(database::init_db());
}
