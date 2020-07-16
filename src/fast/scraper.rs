use regex::Regex;
use reqwest::{blocking::Client, Error};
use select::{document::Document, predicate::Attr};

lazy_static! {
    static ref RE_ENDPOINT: Regex = Regex::new(r#"apiEndpoint="([\w|\\/|\.]*)""#).unwrap();
    static ref RE_TOKEN: Regex = Regex::new(r#"token:"(\w*)""#).unwrap();
    static ref RE_COUNT: Regex = Regex::new(r#"urlCount:(\d*)"#).unwrap();
    static ref RE_PAYLOAD: Regex = Regex::new(r#"MAX_PAYLOAD_BYTES=(\d*)"#).unwrap();
}
const ENDPONT: &str = "https://fast.com";

/// Returns the contents of the javascript file served by Fast
pub fn get_js_file(client: &Client) -> Result<String, Error> {
    let data = client.get(ENDPONT).send()?.text()?;
    let document = Document::from(data.as_str());

    let filename = document
        .find(Attr("src", ()))
        .into_selection()
        .first()
        .expect("No src attribute found")
        .attr("src")
        .unwrap();
    let javascript = client
        .get(format!("{}{}", ENDPONT, filename).as_str())
        .send()?
        .text()?;
    Ok(javascript)
}

/// Parses the javascript for a given regular expression
fn parse_javascript(javascript: &str, re: &Regex) -> String {
    let res = re.captures(javascript).expect("Capture failed");
    res.get(1).expect("Item missing").as_str().to_string()
}

/// Parses the javascript given and returns the api endpoint url
pub fn get_api_endpoint(javascript: &str) -> String {
    format!("https://{}", parse_javascript(javascript, &RE_ENDPOINT))
}

/// Parses the javascript given and returns the api token
pub fn get_token(javascript: &str) -> String {
    parse_javascript(javascript, &RE_TOKEN)
}

/// Parses the javascript given and returns number of urls
pub fn get_url_count(javascript: &str) -> u16 {
    let count_string = parse_javascript(javascript, &RE_COUNT);
    count_string.parse().expect("String s-> u16 parsing failed")
}

/// Parses the javascript given and returns default maximum payload length
pub fn get_max_payload_length(javascript: &str) -> usize {
    let max_string = parse_javascript(javascript, &RE_PAYLOAD);
    max_string.parse().expect("String -> u32 parsing failed")
}
