extern crate rocket;

use routes;

pub fn run() -> () {
    //Mount routes
    let app: rocket::Rocket = routes::collect_routes(rocket::ignite());

    app.launch();
}
