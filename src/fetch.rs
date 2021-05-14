use reqwest::blocking::get;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Payload {
    pub id: String,
    pub price: String,
}

pub fn fetch_price(coin_id: &str) -> Result<Vec<Payload>, reqwest::Error> {
    let endpoint = format!(
        "{}/currencies/ticker?key={}&ids={}",
        dotenv!("API_URL"),
        dotenv!("API_KEY"),
        coin_id
    );

    let response: Vec<Payload> = get(endpoint)?.json()?;
    Ok(response)
}
