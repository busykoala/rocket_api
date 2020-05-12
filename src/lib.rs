#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::collections::HashMap;

#[get("/")]
fn index() -> String {
    format!("Hello, world!")
}

#[get("/<day_number>")]
fn week_day(day_number: u8) -> String {
    let week_days: HashMap<u8, &str> = [
        (1, "Monday"), (2, "Tuesday"), (3, "Wednesday"), (4, "Thursday"),
        (5, "Friday"), (6, "Saturday"), (7, "Sunday")
    ].iter().cloned().collect();

    if let Some(value) = week_days.get(&day_number) {
        format!("Week day {} is {}.", day_number.to_string(), value)
    } else {
        format!("Week day {} does not exist.", day_number.to_string())
    }
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, week_day])
}
