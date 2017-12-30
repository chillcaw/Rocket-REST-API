use urls;
use config;

pub fn run() -> () {
    //Set config
    let init = config::init_app();

    //Mount routes
    let app = urls::collect_urls(init);

    app.launch();
}
