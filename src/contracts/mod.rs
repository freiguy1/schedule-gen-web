
use schedule_gen::contract as lib_contracts;


/*
 * INPUT CONTRACTS
 */

#[derive(RustcDecodable)]
pub struct LeagueSpec {
    pub teams: Vec<(String, String)>,
    pub locations: Vec<(String, String)>,
    pub start_date: Date,
    pub end_date: Date,
    pub game_weekday: GameWeekday
}

impl LeagueSpec {
    pub fn to_lib_type(&self) -> lib_contracts::LeagueSpec {
        lib_contracts::LeagueSpec {
            teams: self.teams.clone(),
            locations: self.locations.clone(),
            start_date: self.start_date.to_lib_type(),
            end_date: self.end_date.to_lib_type(),
            game_weekday: self.game_weekday.to_lib_type()
        }
    }
}

#[derive(RustcDecodable)]
pub struct GameWeekday {
    pub day: Weekday,
    pub game_times: Vec<GameTime>
}

impl GameWeekday {
    fn to_lib_type(&self) -> lib_contracts::GameWeekday {
        lib_contracts::GameWeekday {
            day: self.day.to_lib_type(),
            game_times: self.game_times.iter().map(|gt| gt.to_lib_type()).collect()
        }
    }
}

#[derive(RustcDecodable)]
pub struct GameTime {
    pub time: Time,
    pub location_ids: Vec<String>
}

impl GameTime {
    fn to_lib_type(&self) -> lib_contracts::GameTime {
        lib_contracts::GameTime {
            time: self.time.to_lib_type(),
            location_ids: self.location_ids.clone()
        }
    }
}

#[derive(RustcDecodable)]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

impl Weekday {
    fn to_lib_type(&self) -> lib_contracts::Weekday {
        match *self {
            Weekday::Sunday => lib_contracts::Weekday::Sunday,
            Weekday::Monday => lib_contracts::Weekday::Monday,
            Weekday::Tuesday => lib_contracts::Weekday::Tuesday,
            Weekday::Wednesday => lib_contracts::Weekday::Wednesday,
            Weekday::Thursday => lib_contracts::Weekday::Thursday,
            Weekday::Friday => lib_contracts::Weekday::Friday,
            Weekday::Saturday => lib_contracts::Weekday::Saturday,
        }
    }
}

/*
 * INPUT AND OUTPUT CONTRACTS
 */

#[derive(RustcDecodable, RustcEncodable)]
pub struct Time {
    pub hour: u8,
    pub min: u8
}

impl Time {
    fn to_lib_type(&self) -> lib_contracts::Time {
        lib_contracts::Time {
            hour: self.hour,
            min: self.min
        }
    }
    fn from_lib_type(source: &lib_contracts::Time) -> Time {
        Time {
            hour: source.hour,
            min: source.min
        }
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8
}

impl Date {
    fn to_lib_type(&self) -> lib_contracts::Date {
        lib_contracts::Date {
            year: self.year,
            month: self.month,
            day: self.day
        }
    }
    fn from_lib_type(source: &lib_contracts::Date) -> Date {
        Date {
            year: source.year,
            month: source.month,
            day: source.day
        }
    }
}

/*
 * OUTPUT CONTRACTS
 */

#[derive(RustcEncodable)]
pub struct GameDay {
    day: Date,
    games: Vec<Game>,
    bye_team: Option<(String, String)>
}

impl GameDay {
    pub fn from_lib_type(source: Vec<lib_contracts::TeamEvent>) -> Result<GameDay, &'static str> {
        if source.len() == 0 {
            return Err("No TeamEvents in source");
        }

        let date: &lib_contracts::Date = &source[0].get_date();
        let mut num_byes = 0usize;
        for team_event in source.iter() {
            if team_event.get_date() != *date {
                return Err("Not all TeamEvents on the same Date");
            }
            match *team_event {
                lib_contracts::TeamEvent::Game(_, _, _, _, _) => (),
                lib_contracts::TeamEvent::Bye(_, _) => num_byes += 1
            }
        }

        if num_byes > 1 {
            return Err("More than one bye in source");
        }

        let mut result = GameDay {
            day: Date::from_lib_type(date),
            games: Vec::new(),
            bye_team: None
        };

        for team_event in source.iter() {
            match team_event {
                &lib_contracts::TeamEvent::Game(ref home_team, ref away_team, _, ref time, ref location) => {
                    result.games.push(Game{
                        home_team: home_team.clone(),
                        away_team: away_team.clone(),
                        time: Time::from_lib_type(time),
                        location: location.clone()
                    });
                },
                &lib_contracts::TeamEvent::Bye(ref team, _) => {
                    result.bye_team = Some(team.clone())
                }
            }
        }
        Ok(result)
    }
}

#[derive(RustcEncodable)]
struct Game {
    home_team: (String, String),
    away_team: (String, String),
    time: Time,
    location: (String, String),
}

