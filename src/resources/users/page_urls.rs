use rocket;
use rocket::response::Response;

use config::database::DbConn;
use tools::response;

use resources::users;
use self::users::page_views::PageView;
use self::users::models::PageQuery;

#[get("/users?<page_query>")]
fn page(conn: DbConn, page_query: PageQuery) -> Response<'static> {
    let page = page_query.page;
    let offset = page_query.offset;

    let data = PageView::new(conn, page, offset).page();

    response::Build::new(data, 200)
}
