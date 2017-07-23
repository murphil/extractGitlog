mod conf;
use conf::Config;

fn main() {
    let config = Config::new("/conf.yml");
    println!("{:?}", config);


}
