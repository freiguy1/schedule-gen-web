#[macro_use] extern crate nickel;
extern crate schedule_gen;
extern crate rustc_serialize;

mod contracts;
mod api_controller;

use nickel::Nickel;
use nickel::router::http_router::HttpRouter;


fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();
    api_controller::ApiController::initialize(&mut router);
    router.get("**", middleware!("Hello, World!"));
    server.utilize(router);
    server.listen("0.0.0.0:3000");
}


