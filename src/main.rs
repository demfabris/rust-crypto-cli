extern crate clap;
extern crate reqwest;
extern crate serde_json;

mod app;

fn main() {
    let matches = app::AppRuntime.build().get_matches();
    todo!("{:?}", matches)
}
