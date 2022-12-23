use std::env;
use std::process;
use tailtail::Config;


fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing args: {err}");
        process::exit(1);
    });

    tailtail::run(config);
}
