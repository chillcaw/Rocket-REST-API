use rocket;
use rocket_contrib;

use rocket::response::content;
use rocket::response::status;
use rocket::response::Response;
use rocket_contrib::{Json, Value};

use resources::users;
use self::users::views;
use self::users::models::User;

#[get("/")]
fn all() -> Json<Value> {
    return views::all();
}

#[get("/<id>")]
fn find(id: usize) -> Json<Value> {
    return views::find(id);
}

#[post("/", format = "application/json", data = "<user>")]
fn create(user: Json<User>) -> Json<Value> {
    return views::create(user.into_inner());
}

#[put("/<id>", format = "application/json", data = "<user>")]
fn update(id: usize, user: Json<User>) -> Json<Value> {
    return views::update(id, user.into_inner());
}

#[delete("/<id>")]
fn delete(id: usize) -> Json<Value> {
    return views::delete(id);
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
