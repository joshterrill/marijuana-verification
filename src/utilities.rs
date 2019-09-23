use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};

use crate::providers::Error;
// Option 1

pub trait ResultToJson {
    fn to_json(&self) -> Json<JsonValue>;
}

impl<T: serde::Serialize> ResultToJson for Result<T, Error> {
    fn to_json(&self) -> Json<JsonValue> {
        match self {
            Ok(content) => Json(json!(content)),
            Err(error) => Json(json!({ "error": error.message })),
        }
    }
}
