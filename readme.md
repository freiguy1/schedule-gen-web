## A REST web front end for generating a league schedule.

### Technologies Used:
1. [schedule-gen-rs](https://github.com/freiguy1/schedule-gen-rs) - league schedule generator library
2. [nickel.rs](https://github.com/nickel-org/nickel.rs) - rust web framework
3. [rust language](http://rust-lang.org) - rust language

### Instructions:
1. Clone repo: `git clone https://github.com/freiguy1/schedule-gen-web`
2. Run server: `cd schedule-gen-web && cargo run`
3. Post a json league specification to http://localhost:3000; an example spec if provided below
4. ...
5. Profit

### Example League Specification:
```json
{
  "teams" : [
    ["0", "teamname0"],
    ["1", "teamname1"],
    ["2", "teamname2"],
    ["3", "teamname3"],
    ["4", "teamname4"]
  ],
  "locations" : [
    ["0", "field0"],
    ["1", "field1"]
  ],
  "start_date" : {
    "day": 16,
    "month": 9,
    "year": 2014
  },
  "end_date" : {
    "day": 23,
    "month": 12,
    "year": 2014
  },
  "game_weekday" : {
    "day": "Tuesday",
    "game_times": [
      {
        "time": {
          "hour": 16,
          "min": 0
        },
        "location_ids": ["0"]
      },
      {
        "time": {
          "hour": 17,
          "min": 0
        },
        "location_ids": ["0", "1"]
      }
     ]
  }
}
```
