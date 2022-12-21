use std::env;

struct Config {
    file_path: String,
    number_of_lines: i32,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // number of lines should be optional
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let file_path = args[1].clone();
        println!("File Path: {file_path}");
        let number_of_lines: i32 = if args.len() == 3 {
            match args[2].clone().parse() {
                Ok(num) => num,
                Err(_) => 10,
            }
        } else {
            10
        };
        println!("Number of lines: {number_of_lines}");

        Ok(Config {file_path, number_of_lines})
    }
}

fn main() {
    // read in configs
    // parse configs
    // use configs to get the last n lines
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);
}


