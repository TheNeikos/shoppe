#![feature(plugin)]
#![plugin(maud_macros)]
#![feature(custom_derive, custom_attribute)]
#![plugin(diesel_codegen, dotenv_macros)]


extern crate iron;
extern crate router;
extern crate mount;
extern crate logger;
extern crate maud;
extern crate params;
#[macro_use] extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate lazy_static;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate bcrypt;

use iron::prelude::*;
use iron::status;
use router::Router;
use mount::Mount;
use logger::Logger;

#[macro_use]
mod macros;
mod database;
mod controllers;
mod views;
mod error;
mod models;

fn main() {
    use controllers::user;
    let mut index_router = Router::new();
    index_router.get("/", controllers::root::handler);
    index_router.get("/about", controllers::about::handler);

    let user_router = resource![user];

    let mut mount = Mount::new();
    mount.mount("/", index_router)
         .mount("/users", user_router);

    let (log_bef, log_aft) = Logger::new(Some(logger::format::Format::default()));

    let mut log_chain = Chain::new(mount);
    log_chain.link_before(log_bef);

    log_chain.link_after(log_aft);
    Iron::new(log_chain).http("0.0.0.0:3000").unwrap();
}

