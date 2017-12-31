use rocket;
use rocket_contrib::{Json, Value};

use config::database::DbConn;
use resources::users;
use self::users::views::View;
use self::users::models::User;

#[get("/")]
fn all(conn: DbConn) -> Json<Value> {
    return View::new(conn).all();
}

#[get("/<id>")]
fn find(id: i32, conn: DbConn) -> Json<Value> {
    return View::new(conn).find(id);
}

#[post("/", format = "application/json", data = "<user>")]
fn create(user: Json<User>, conn: DbConn) -> Json<Value> {
    return View::new(conn).create(user.into_inner());
}

#[put("/<id>", format = "application/json", data = "<user>")]
fn update(id: i32, user: Json<User>, conn: DbConn) -> Json<Value> {
    return View::new(conn).update(id, user.into_inner());
}

#[delete("/<id>")]
fn delete(id: i32, conn: DbConn) -> Json<Value> {
    return View::new(conn).delete(id);
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
