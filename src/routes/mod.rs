extern crate rocket;

pub mod users;

pub fn collect_routes(app: rocket::Rocket) -> rocket::Rocket {
    return app.mount("/users/", users::get_routes());
}
