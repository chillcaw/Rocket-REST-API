use rocket;

pub mod views;
pub mod models;
pub mod urls;
pub mod serializers;

pub mod schema {
    table! {
        users (id) {
            id -> Int4,
            name -> Varchar,
        }
    }
}

pub fn collect_urls() -> Vec<rocket::Route> {
    return urls::get_urls();
}
