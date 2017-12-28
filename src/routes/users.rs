extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;

use rocket_contrib::{Json, Value};

use resources::users;
use self::users::controller;
use self::users::model::User;

#[get("/")]
fn all() -> &'static str {
    return controller::all();
}

#[get("/<id>")]
fn find(id: usize) -> &'static str {
    return controller::find();
}

#[post("/", format = "application/json", data = "<user>")]
fn create(user: Json<User>) -> &'static str {
    return controller::create()
}

#[delete("/<id>")]
fn delete(id: usize) -> &'static str {
    return controller::delete();
}

#[put("/<id>", format = "application/json", data = "<user>")]
fn update(id: u8, user: Json<User>) -> &'static str {
    return controller::update();
}

pub fn get_routes() -> Vec<rocket::Route> {
    return routes![
        all,
        find,
        create,
        delete,
        update
    ];
}
