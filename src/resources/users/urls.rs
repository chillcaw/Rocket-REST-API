use rocket;
use rocket_contrib;

use rocket_contrib::{Json, Value};

use resources::users;
use self::users::views;
use self::users::models::User;

#[get("/")]
fn all() -> &'static str {
    return views::all();
}

#[get("/<id>")]
fn find(id: usize) -> &'static str {
    return views::find();
}

#[post("/", format = "application/json", data = "<user>")]
fn create(user: Json<User>) -> &'static str {
    return views::create()
}

#[put("/<id>", format = "application/json", data = "<user>")]
fn update(id: u8, user: Json<User>) -> &'static str {
    return views::update();
}

#[delete("/<id>")]
fn delete(id: usize) -> &'static str {
    return views::delete();
}


pub fn get_urls() -> Vec<rocket::Route> {
    return routes![
        all,
        find,
        create,
        update,
        delete
    ];
}
