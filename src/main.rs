mod config;

use config::Config;

fn main() {
    let config_path = "config.json";
    let config = Config::init(config_path);

    println!("{:?}", &config.intervals_before_long_break);
}
