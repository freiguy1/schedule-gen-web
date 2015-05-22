#[macro_use] extern crate nickel;
extern crate schedule_gen;
extern crate rustc_serialize;
extern crate hyper;

mod contracts;
mod api_controller;

use hyper::header::Location;
use nickel::{ StaticFilesHandler, Nickel };
use nickel::router::http_router::HttpRouter;
use std::collections::HashMap;

const GITHUB_URL: &'static str = "https://github.com/freiguy1/schedule-gen-web";


fn main() {
    let mut server = Nickel::new();
    
    // Enable Router
    let mut router = Nickel::router();
    api_controller::ApiController::initialize(&mut router);
    router.get("/github", middleware! { |request, mut response| 
        response.origin.headers_mut().set(Location(GITHUB_URL.into()));
        (nickel::status::StatusCode::TemporaryRedirect, "")
    });

    router.get("/", middleware! { |_req, res|
        let data = HashMap::<&str, &str>::new();
        return res.render("templates/index.tpl", &data);
    });

    // router.get("/assets/*", StaticFilesHandler::new("assets/"));

    server.utilize(StaticFilesHandler::new("public/"));

    server.utilize(router);

    server.listen("0.0.0.0:6767");
}


