use super::error::FastError;
use regex::Regex;
use reqwest::Client;
use select::{document::Document, predicate::Attr};
use std::io::{Error as IoError, ErrorKind::NotFound};

lazy_static! {
    static ref RE_ENDPOINT: Regex = Regex::new(r#"apiEndpoint="([\w|\\/|\.]*)""#).unwrap();
    static ref RE_TOKEN: Regex = Regex::new(r#"token:"(\w*)""#).unwrap();
    static ref RE_COUNT: Regex = Regex::new(r#"urlCount:(\d*)"#).unwrap();
    static ref RE_PAYLOAD: Regex = Regex::new(r#"MAX_PAYLOAD_BYTES=(\d*)"#).unwrap();
}
const ENDPONT: &str = "https://fast.com";

/// Returns the contents of the javascript file served by Fast
pub async fn get_js_file(client: &Client) -> Result<String, FastError> {
    let res = client.get(ENDPONT).send().await?;
    let data = res.text().await?;
    let document = Document::from(data.as_str());

    let filename = document
        .find(Attr("src", ()))
        .into_selection()
        .first()
        .ok_or(IoError::new(NotFound, "src Attribute Not Found"))?
        .attr("src")
        .unwrap();
    let javascript = client
        .get(format!("{}{}", ENDPONT, filename).as_str())
        .send()
        .await?
        .text()
        .await?;
    Ok(javascript)
}

/// Parses the javascript for a given regular expression
fn parse_javascript(javascript: &str, re: &Regex) -> Result<String, FastError> {
    Ok(re
        .captures(javascript)
        .ok_or(IoError::new(NotFound, "No Match Found"))?
        .get(1)
        .ok_or(IoError::new(NotFound, "First Match Not Found"))?
        .as_str()
        .to_string())
}

/// Parses the javascript given and returns the api endpoint url
pub fn get_api_endpoint(javascript: &str) -> Result<String, FastError> {
    Ok("https://".to_string() + &parse_javascript(javascript, &RE_ENDPOINT)?)
}

/// Parses the javascript given and returns the api token
pub fn get_token(javascript: &str) -> Result<String, FastError> {
    Ok(parse_javascript(javascript, &RE_TOKEN)?)
}

/// Parses the javascript given and returns number of urls
pub fn get_url_count(javascript: &str) -> Result<u16, FastError> {
    Ok(parse_javascript(javascript, &RE_COUNT)?.parse()?)
}

/// Parses the javascript given and returns default maximum payload length
pub fn get_max_payload_length(javascript: &str) -> Result<usize, FastError> {
    let max_string = parse_javascript(javascript, &RE_PAYLOAD)?;
    Ok(max_string.parse()?)
}
