use rocket;

pub mod models;
pub mod urls;
pub mod page_urls;
pub mod views;
pub mod page_views;
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
