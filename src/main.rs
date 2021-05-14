extern crate clap;
extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate dotenv_codegen;

mod app;
mod fetch;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let matches = app::AppRuntime.build().get_matches();
    let coin_id = matches.value_of("COIN").expect("No coin id provided");

    let data = fetch::fetch_price(coin_id).expect("Failed to fetch data");

    if data.is_empty() {
        println!("Invalid coin ID");
    } else {
        let payload = data.first().unwrap();
        let price = payload.price.parse::<f32>().unwrap();

        println!("Price of {} is currently: ${:.2}", &payload.id, price);
    }
}
