use clap::{App, Arg};

pub struct AppRuntime;

impl AppRuntime {
    pub fn build(&self) -> App<'static, 'static> {
        App::new("Rust CLI Crypto")
            .version("0.1.0")
            .author("Fabricio7p <fabricio7p@protonmail.com>")
            .about("Fetch quick and dirty crypto info")
            .arg(
                Arg::with_name("version")
                    .short("v")
                    .long("version")
                    .help("Shows current version"),
            )
            .arg(
                Arg::with_name("COIN")
                    .short("c")
                    .long("coin")
                    .help("Coin ")
                    .index(1),
            )
    }
}
