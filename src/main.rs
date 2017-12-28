#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

mod routes;
mod app;
mod resources;

fn main() {
    app::run();
}
