use serde::Deserialize;

#[derive(Deserialize)]
/// JSON data from Fast API
pub struct FastJson {
    pub client: FastClient,
    pub targets: Vec<FastTarget>,
}

#[derive(Deserialize)]
/// Data about the user
pub struct FastClient {
    pub asn: String,
    pub ip: String,
    pub isp: String,
}
#[derive(Deserialize)]
/// Target to where we send packets
pub struct FastTarget {
    pub location: FastLocation,
    pub name: String,
    pub url: String,
}
#[derive(Deserialize)]
/// A geographical location
pub struct FastLocation {
    pub city: String,
    pub country: String,
}
