use clap::{Arg, Command};
use std::{
    env,
    io::{self, BufRead},
    process, thread,
    time::Duration,
};

fn main() {
    // Set up command-line arguments using Clap
    let matches = Command::new("slomore")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A custom pager that outputs lines with a delay, allowing control over pacing.")
        .arg(
            Arg::new("seconds_per_line")
                .short('s')
                .long("seconds-per-line")
                .value_name("SECONDS")
                .value_parser(clap::value_parser!(f64))
                .conflicts_with("lines_per_second")
                .help("Set delay in seconds between lines. Must be greater than 0."),
        )
        .arg(
            Arg::new("lines_per_second")
                .short('l')
                .long("lines-per-second")
                .value_name("LINES")
                .value_parser(clap::value_parser!(f64))
                .conflicts_with("seconds_per_line")
                .help("Set the number of lines to display per second. Must be greater than 0."),
        )
        .arg(
            Arg::new("initial_lines")
                .short('i')
                .long("initial-lines")
                .value_name("LINES")
                .value_parser(clap::value_parser!(usize))
                .help("Set the number of initial lines to display without delay."),
        )
        .get_matches();

    // Function to check that a value is greater than 0
    fn validate_positive(value: f64, name: &str) {
        if value <= 0.0 {
            eprintln!("Error: {} must be greater than 0", name);
            process::exit(1);
        }
    }

    // Determine delay, prioritizing command-line options, then environment variables, then default
    let delay = if let Some(seconds_per_line) = matches.get_one::<f64>("seconds_per_line") {
        validate_positive(*seconds_per_line, "seconds-per-line");
        Duration::from_secs_f64(*seconds_per_line)
    } else if let Some(lines_per_second) = matches.get_one::<f64>("lines_per_second") {
        validate_positive(*lines_per_second, "lines-per-second");
        Duration::from_secs_f64(1.0 / *lines_per_second)
    } else if let Ok(seconds_per_line) = env::var("SLOMORE_SECONDS_PER_LINE") {
        let seconds: f64 = seconds_per_line
            .parse()
            .expect("Invalid number in SLOMORE_SECONDS_PER_LINE");
        validate_positive(seconds, "SLOMORE_SECONDS_PER_LINE");
        Duration::from_secs_f64(seconds)
    } else if let Ok(lines_per_second) = env::var("SLOMORE_LINES_PER_SECOND") {
        let lines: f64 = lines_per_second
            .parse()
            .expect("Invalid number in SLOMORE_LINES_PER_SECOND");
        validate_positive(lines, "SLOMORE_LINES_PER_SECOND");
        Duration::from_secs_f64(1.0 / lines)
    } else {
        // Default to 10 lines per second
        Duration::from_secs_f64(1.0 / 10.0)
    };

    // Determine how many lines to show immediately before applying the delay
    let initial_lines = if let Some(initial_lines) = matches.get_one::<usize>("initial_lines") {
        *initial_lines
    } else if let Ok(initial_lines) = env::var("SLOMORE_INITIAL_LINES") {
        initial_lines
            .parse()
            .expect("Invalid number in SLOMORE_INITIAL_LINES")
    } else {
        0
    };

    // Read from stdin and output with delay
    let stdin = io::stdin();
    let handle = stdin.lock();
    for (index, line) in handle.lines().enumerate() {
        if let Ok(line) = line {
            println!("{}", line);
            if index + 1 > initial_lines {
                thread::sleep(delay);
            }
        } else {
            eprintln!("Error reading line");
            break;
        }
    }
}
