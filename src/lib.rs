use std::fs;

pub struct Config {
    pub file_path: String,
    pub number_of_lines: i32,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let file_path = args[1].clone();
        let number_of_lines: i32 = if args.len() == 3 {
            match args[2].clone().parse() {
                Ok(num) => num,
                Err(_) => 10,
            }
        } else {
            10
        };

        Ok(Config {file_path, number_of_lines})
    }
}

pub fn run(config: Config) {
    let mut contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    let mut line_break_count = 0;
    let mut output_string = String::new();
    while let Some(ch) = contents.pop() {
        if ch == '\n' {
            line_break_count += 1;
        }
        if line_break_count > config.number_of_lines {
            break;
        }
        output_string.insert(0, ch);
    }

    println!("{output_string}");
}