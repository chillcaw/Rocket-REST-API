use rocket;
use rocket::response::Response;
use rocket_contrib::Json;

use config::database::DbConn;
use tools::response;

use resources::users;
use self::users::views::View;
use self::users::models::NewUser;
use self::users::page_urls;

#[get("/users")]
fn all(conn: DbConn) -> Response<'static> {
    let data = View::new(conn).all();

    response::Build::new(data, 200)
}

#[get("/users/<id>")]
fn find(id: i32, conn: DbConn) -> Response<'static> {
    let data = View::new(conn).find(id);

    response::Build::new(data, 200)
}

#[post("/users", format = "application/json", data = "<user>")]
fn create(user: Json<NewUser>, conn: DbConn) -> Response<'static> {
    let data = View::new(conn).create(user.into_inner());

    response::Build::new(data, 201)
}

#[put("/users/<id>", format = "application/json", data = "<user>")]
fn update(id: i32, user: Json<NewUser>, conn: DbConn) -> Response<'static> {
    let data = View::new(conn).update(id, user.into_inner());

    response::Build::new(data, 200)
}

#[delete("/users/<id>")]
fn delete(id: i32, conn: DbConn) -> Response<'static> {
    let data = View::new(conn).delete(id);

    response::Build::new(data, 200)
}

pub fn get_urls() -> Vec<rocket::Route> {
    return routes![
        page_urls::page,
        all,
        find,
        create,
        update,
        delete
    ];
}
