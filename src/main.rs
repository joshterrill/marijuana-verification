#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use scraper::{Html, Selector};
extern crate reqwest;


#[get("/<rec_id>")]
fn index(rec_id: &RawStr) -> String {
    verify_green_life(rec_id.to_string())
}

fn get_html_from_url(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(&url)?.text()?;
    Ok(resp)
}

fn verify_green_life(rec_id: String) -> String {

  let mut green_life_url = "https://verify.greenlifemedical.com/recommendations?utf8=%E2%9C%93&rec_id=".to_owned();

  green_life_url.push_str(&rec_id);

  let html = get_html_from_url(green_life_url).unwrap();
    // println!("{:?}", html);
    let document = Html::parse_document(&html);
    let selector = Selector::parse("div.result").unwrap();

    match document.select(&selector).next() {
      Some(item) => {
        item.inner_html()
      },
      None => {
        format!("Nothing found")
      }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
