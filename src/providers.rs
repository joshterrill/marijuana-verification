extern crate reqwest;
use scraper::{Html, Selector};

#[derive(Serialize, Deserialize, Debug)]
pub struct GreenLifeMedical {
  pub status: String,
  pub patient_initials: String,
  pub issue_date: String,
  pub valid_through: String,
  pub doctors_name: String,
}

pub fn get_html_from_url(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(&url)?.text()?;
    Ok(resp)
}

pub fn verify_green_life(rec_id: String) -> Result<GreenLifeMedical, String> {

  let mut green_life_url = "https://verify.greenlifemedical.com/recommendations?utf8=%E2%9C%93&rec_id=".to_owned();

  green_life_url.push_str(&rec_id);

  let html = get_html_from_url(green_life_url).unwrap();
    // println!("{:?}", html);
    let document = Html::parse_document(&html);
    let selector = Selector::parse("div.result").unwrap();

    match document.select(&selector).next() {
      Some(item) => {
        let resp = item.inner_html();
        let mut iter = resp.split("<br>");
        let status = iter.nth(1).unwrap().trim().split(" is " ).nth(1).unwrap().to_string();
        let patient_initials = iter.nth(0).unwrap().trim().split("Patient Initials: ").nth(1).unwrap().to_string();
        let issue_date = iter.nth(0).unwrap().trim().split("Issue Date: ").nth(1).unwrap().to_string();
        let valid_through = iter.nth(0).unwrap().trim().split("Valid Through: ").nth(1).unwrap().to_string();
        let doctors_name = iter.nth(0).unwrap().trim().split("Doctors Name: ").nth(1).unwrap().to_string();
        let patient = GreenLifeMedical {
          status,
          patient_initials,
          issue_date,
          valid_through,
          doctors_name,
        };
        Ok(patient)
      },
      None => {
        Err(format!("Patient not found"))
      }
    }
}