use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{value_parser, Arg, ArgAction, Command};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank: bool,
}

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Delvoid")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .num_args(1..)
                .value_parser(value_parser!(String))
                .help("input file(s)")
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number lines")
                .conflicts_with("number_nonblank")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .help("Number non-blank lines")
                .conflicts_with("number")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .map(|f| f.map(|s| s.to_string()).collect::<Vec<_>>()) // Convert &OsString to String
            .unwrap_or_default(),
        number_lines: matches.get_flag("number"),
        number_nonblank: matches.get_flag("number_nonblank"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open_file(&filename) {
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    match line_result {
                        Ok(line) => {
                            if config.number_lines {
                                println!("{:6}\t{}", line_num + 1, line);
                            } else if config.number_nonblank {
                                if line.is_empty() {
                                    println!("{}", line);
                                } else {
                                    last_num += 1;
                                    println!("{:6}\t{}", last_num, line);
                                }
                            } else {
                                println!("{}", line);
                            }
                        }
                        Err(e) => eprintln!("{}: {}", filename, e),
                    }
                }
            }
            Err(e) => eprintln!("{}: {}", filename, e),
        }
    }
    Ok(())
}

fn open_file(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
