extern crate reqwest;
use scraper::{Html, Selector};

#[derive(Serialize, Deserialize)]
pub struct GreenLifeMedical {
  pub status: String,
  pub patient_initials: String,
  pub issue_date: String,
  pub valid_through: String,
  pub doctors_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct PatientIdCenter {
  pub status: String,
}

pub fn verify_patient_id_center(member_number: &str, id_number: &str) -> Result<PatientIdCenter, String> {
  let patient_id_center_url = "http://verify.rxcbc.org/index.php".to_owned();
  let params = [("sMemberNumber", member_number), ("sIDNumber", id_number), ("btnSubmit", "Submit")];
  let html = post_html_from_url(patient_id_center_url, params.to_vec()).unwrap();
  let document = Html::parse_document(&html);
  let selector = Selector::parse("#lblStatus2 font font").unwrap();
  match document.select(&selector).next() {
    Some(found_selector) => {
      let inner_html = found_selector.inner_html();
      match inner_html != "NOT FOUND" {
        true => {
          // todo: need to find a real patient to parse patient data
          let patient = PatientIdCenter {
            status: "ACTIVE".to_string(),
          };
          Ok(patient)
        },
        _ => {
          Err(format!("Patient not found"))
        }
      }
    },
    None => {
      Err(format!("Patient not found"))
    }
  }
}

pub fn verify_green_life(rec_id: &str) -> Result<GreenLifeMedical, String> {

  let mut green_life_url = "https://verify.greenlifemedical.com/recommendations?utf8=%E2%9C%93&rec_id=".to_owned();

  green_life_url.push_str(&rec_id);

  let html = get_html_from_url(green_life_url).unwrap();
  let document = Html::parse_document(&html);
  let selector = Selector::parse("div.result").unwrap();

  match document.select(&selector).next() {
    Some(found_selector) => {
      let inner_html = found_selector.inner_html();
      let mut iter = inner_html.split("<br>");
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

fn get_html_from_url(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(&url)?.text()?;
    Ok(resp)
}

fn post_html_from_url(url: String, params: Vec<(&str, &str)>) -> Result<String, Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let resp = client.post(&url)
    .form(&params)
    .send()?.text()?;
  Ok(resp)
}