extern crate rocket;

use routes;
use config;

pub fn run() -> () {
    let init = config::init_app();

    //Mount routes
    let app: rocket::Rocket = routes::collect_routes(init);

    app.launch();
}
