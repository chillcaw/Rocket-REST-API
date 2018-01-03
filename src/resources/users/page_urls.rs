use rocket::response::Response;

use config::database::DbConn;
use tools::response;

use resources::users;
use self::users::page_views::PageView;
use self::users::models::PageQuery;

#[get("/users?<page_query>")]
fn page(conn: DbConn, page_query: PageQuery) -> Response<'static> {
    let page = page_query.page.unwrap_or(1);
    let offset = page_query.offset.unwrap_or(2);

    let data = PageView::new(conn, page, offset).page();

    response::Build::new(data, 200)
}
