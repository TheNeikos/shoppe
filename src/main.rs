#![feature(concat_idents)]


extern crate iron;
extern crate router;
extern crate mount;

use iron::prelude::*;
use iron::status;
use router::Router;
use mount::Mount;

#[macro_use]
mod macros;
mod controllers;
mod error;

fn main() {
    use controllers::user;
    let mut index_router = Router::new();
    index_router.get("/", controllers::root::handler);
    index_router.get("/about", controllers::about::handler);

    let user_router = resource![user];

    let mut mount = Mount::new();
    mount.mount("/", index_router)
         .mount("/user/", user_router);

    Iron::new(mount).http("0.0.0.0:3000").unwrap();
}

