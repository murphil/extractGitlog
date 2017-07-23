mod conf;
use conf::Config;
#[macro_use]
extern crate serde_derive;

fn main() {
    let config = Config::new("./conf.yml").unwrap();
    //config.map(|x| x);
    println!("{:?}", config.tags.get("U+"));


}
