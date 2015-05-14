use nickel::{ Router, MediaType, JsonBody };
use nickel::router::http_router::HttpRouter;
use nickel::status::StatusCode::{ BadRequest };
use rustc_serialize;
use schedule_gen::contract as lib_contracts;
use schedule_gen;
use std::error::Error;

pub struct ApiController;

impl ApiController{
    pub fn initialize(router: &mut Router) {

        // GENERATE SCHEDULE
        router.post("/", middleware! { | request, mut response |
            let league_spec = match request.json_as::<::contracts::LeagueSpec>() {
                Ok(league_spec) => league_spec,
                Err(error) => {
                    response.set_status(BadRequest);
                    return response.send(rustc_serialize::json::encode(&vec![error.description()]).unwrap());
                }
            };

            let team_events = match schedule_gen::generate_games(&league_spec.to_lib_type()) {
                Ok(team_events) => team_events,
                Err(errors) => {
                    response.set_status(BadRequest);
                    return response.send(rustc_serialize::json::encode(&errors).unwrap());
                }
            };
            let mut grouped_team_events: Vec<Vec<lib_contracts::TeamEvent>> = Vec::new();
            for team_event in team_events {
                if grouped_team_events.iter().any(|group| group[0].get_date() == team_event.get_date()) {
                    grouped_team_events.iter_mut().find(|group| group[0].get_date() == team_event.get_date()).unwrap().push(team_event);
                } else {
                    grouped_team_events.push(vec![team_event]);
                }
            }
            let game_dates: Vec<::contracts::GameDay> = grouped_team_events.into_iter().map(|group| ::contracts::GameDay::from_lib_type(group).ok().unwrap()).collect();
            response.content_type(MediaType::Json);
            rustc_serialize::json::encode(&game_dates).unwrap()
        });


    }
}


