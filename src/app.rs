extern crate rocket;

use routes;
use config::database;

pub fn run() -> () {
    let init = rocket::ignite()
        .manage(database::connect_db());

    //Mount routes
    let app: rocket::Rocket = routes::collect_routes(init);

    app.launch();
}
