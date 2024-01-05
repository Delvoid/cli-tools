use clap::{value_parser, Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Delvoid")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .num_args(1..)
                .required(true)
                .value_parser(value_parser!(String))
                .help("Input text"),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text_values = matches
        .get_many::<String>("text")
        .map(|values| values.map(|s| s.to_string()).collect::<Vec<_>>()) // Convert &OsString to String
        .unwrap_or_default(); // Provide a default empty Vec if there are no values

    let omit_newline = matches.get_flag("omit_newline");

    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text_values.join(" "), ending);
}
