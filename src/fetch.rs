use reqwest::blocking::get;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Payload {
    pub id: String,
    pub price: String,
}

pub trait PrettyPrintable {
    fn as_pretty(&self);
}

impl PrettyPrintable for Payload {
    fn as_pretty(&self) {
        let price = self.price.parse::<f32>().unwrap();
        println!("Price of {} is currently: ${:.2}", &self.id, price);
    }
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_return_empty_array_if_invalid_id() {
        let response = fetch_price("RABADABA").unwrap();
        assert_eq!(response, []);
    }

    #[test]
    fn response_should_be_single_element_array() {
        let response: Vec<Payload> = fetch_price("ETH").unwrap();
        assert_eq!(response.len(), 1);
    }
}
