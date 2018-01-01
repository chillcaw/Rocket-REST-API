#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

extern crate r2d2_diesel;
extern crate r2d2;
extern crate dotenv;

mod urls;
mod app;
mod resources;
mod config;
mod tools;

fn main() {
    app::run();
}
