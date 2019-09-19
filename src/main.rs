#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket::http::RawStr;
use rocket_contrib::json::{Json};

mod providers;
use providers::{GreenLifeMedical, verify_green_life};

#[get("/")]
fn index() -> &'static str {
  "Marijuana Verification"
}

#[get("/<rec_id>")]
fn green_life(rec_id: &RawStr) -> Json<Result<GreenLifeMedical, String>> {
    Json(verify_green_life(rec_id.to_string()))
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/greenlife", routes![green_life])
    .launch();
}
