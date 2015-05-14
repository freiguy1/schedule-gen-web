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
    router.get("**", middleware! { |request, mut response| 
        response.set_status(nickel::status::StatusCode::TemporaryRedirect);
        response.origin.headers_mut().set_raw("location", vec![b"https://github.com/freiguy1/schedule-gen-web".to_vec()]);
        return response.send("");
    });
    
    /*(nickel::status::StatusCode::TemporaryRedirect, "Redirecting to github", vec!["https://github.com/freiguy1/schedule-gen-web"])));
     */
    server.utilize(router);
    server.listen("0.0.0.0:3000");
}


