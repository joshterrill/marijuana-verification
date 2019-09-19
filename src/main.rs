#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;
use rocket::http::RawStr;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

mod providers;
use providers::{verify_green_life, verify_patient_id_center};

#[get("/")]
fn index() -> Template {
  let version = env!("CARGO_PKG_VERSION");
  let mut context = HashMap::new();
  context.insert("version", version);
  Template::render("index", &context)
}

#[get("/<rec_id>")]
fn green_life(rec_id: &RawStr) -> Json<JsonValue> {
    match verify_green_life(rec_id) {
      Ok(patient) => {
        Json(json!(patient))
      },
      Err(error) => {
        Json(json!({"error": error}))
      }
    }
}

#[get("/<member_number>/<id_number>")]
fn patient_id_center(member_number: &RawStr, id_number: &RawStr) -> Json<JsonValue> {
    match verify_patient_id_center(member_number, id_number) {
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
    .mount("/greenlife", routes![green_life])
    .mount("/patientidcenter", routes![patient_id_center])
    .mount("/", routes![index])
    .attach(Template::fairing())
    .launch();
}
