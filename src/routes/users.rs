extern crate rocket;

use resources::users::controller;

#[get("/")]
fn all() -> &'static str {
    return controller::all();
}

#[get("/<id>")]
fn find(id: u8) -> &'static str {
    return controller::find();
}

// #[post("/")]
// fn create() -> &'static str {
//     return controller::create()
// }

#[delete("/<id>")]
fn delete(id: u8) -> &'static str {
    return controller::delete();
}

// #[put("/<id>")]
// fn update(id: u8) -> &'static str {
//     return controller::update();
// }

pub fn get_routes() -> Vec<rocket::Route> {
    return routes![
        all,
        find,
        //create,
        delete,
        //update
    ];
}
