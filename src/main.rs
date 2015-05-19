#[macro_use] extern crate nickel;
extern crate schedule_gen;
extern crate rustc_serialize;
extern crate hyper;

mod contracts;
mod api_controller;

use hyper::header::Location;
use nickel::Nickel;
use nickel::router::http_router::HttpRouter;


fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();
    api_controller::ApiController::initialize(&mut router);
    router.get("**", middleware! { |request, mut response| 
        response.origin.headers_mut().set(Location("https://github.com/freiguy1/schedule-gen-web".into()));
        (nickel::status::StatusCode::TemporaryRedirect, "")
    });
    
    server.utilize(router);
    server.listen("0.0.0.0:6767");
}


