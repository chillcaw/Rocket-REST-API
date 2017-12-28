#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

mod routes;
mod app;
mod resources;

fn main() {
    app::run();
}
