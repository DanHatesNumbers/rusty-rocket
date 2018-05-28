#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;

extern crate rustyrocket_lib;

use dotenv::dotenv;

mod routing;
mod handlers;

fn main() {
    dotenv().ok();
    routing::create_routes();
}