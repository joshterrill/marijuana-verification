#![feature(proc_macro_hygiene, decl_macro)]

mod providers;
mod utilities;

use providers::{verify_green_life, verify_patient_id_center};
use rocket::http::RawStr;
use rocket::{get, routes};
use rocket_contrib::json::{Json, JsonValue};
use utilities::ResultToJson;
use rocket_contrib::serve::StaticFiles;

#[get("/<rec_id>")]
fn green_life(rec_id: &RawStr) -> Json<JsonValue> {
    verify_green_life(rec_id).to_json()
}

#[get("/<member_number>/<id_number>")]
fn patient_id_center(member_number: &RawStr, id_number: &RawStr) -> Json<JsonValue> {
    verify_patient_id_center(member_number, id_number).to_json()
}

fn main() {
    rocket::ignite()
        .mount("/greenlife", routes![green_life])
        .mount("/patientidcenter", routes![patient_id_center])
        .mount("/", StaticFiles::from("static"))
        .launch();
}
