#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::http::RawStr;
use rocket_contrib::json::{Json, JsonValue};

mod providers;
use providers::{verify_green_life};

#[get("/")]
fn index() -> &'static str {
  "Marijuana Verification"
}

#[get("/<rec_id>")]
fn green_life(rec_id: &RawStr) -> Json<JsonValue> {
    match verify_green_life(rec_id.to_string()) {
      Ok(patient) => {
        Json(json!(patient))
      },
      Err(error) => {
        Json(json!({"error": error}))
      }
    }
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/greenlife", routes![green_life])
    .launch();
}
