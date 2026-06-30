use clap::{Arg, Command};
use std::{
    env::{self, VarError},
    io::{self, BufRead, Write},
    process, thread,
    time::Duration,
};

const DEFAULT_LINES_PER_SECOND: f64 = 10.0;

fn is_positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn validate_positive(value: f64, name: &str) {
    if !is_positive(value) {
        exit_with_error(&format!("{name} must be greater than 0"));
    }
}

fn compute_delay(lines_per_second: f64, name: &str) -> Duration {
    validate_positive(lines_per_second, name);
    Duration::from_secs_f64(1.0 / lines_per_second)
}

fn default_delay() -> Duration {
    compute_delay(DEFAULT_LINES_PER_SECOND, "lines-per-second")
}

fn parse_env_f64(name: &str) -> Option<f64> {
    parse_env_value(name, "number")
}

fn parse_env_usize(name: &str) -> Option<usize> {
    parse_env_value(name, "whole number")
}

fn parse_env_value<T>(name: &str, expected: &str) -> Option<T>
where
    T: std::str::FromStr,
{
    match env::var(name) {
        Ok(value) => match value.parse::<T>() {
            Ok(parsed) => Some(parsed),
            Err(_) => exit_with_error(&format!("{name} must be a valid {expected}")),
        },
        Err(VarError::NotPresent) => None,
        Err(_) => exit_with_error(&format!("{name} contains invalid Unicode")),
    }
}

fn exit_with_error(message: &str) -> ! {
    eprintln!("Error: {message}");
    process::exit(1);
}

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

    // Determine delay, prioritizing command-line options, then environment variables, then default
    let delay = if let Some(seconds_per_line) = matches.get_one::<f64>("seconds_per_line") {
        validate_positive(*seconds_per_line, "seconds-per-line");
        Duration::from_secs_f64(*seconds_per_line)
    } else if let Some(lines_per_second) = matches.get_one::<f64>("lines_per_second") {
        compute_delay(*lines_per_second, "lines-per-second")
    } else if let Some(seconds) = parse_env_f64("SLOMORE_SECONDS_PER_LINE") {
        validate_positive(seconds, "SLOMORE_SECONDS_PER_LINE");
        Duration::from_secs_f64(seconds)
    } else if let Some(lines) = parse_env_f64("SLOMORE_LINES_PER_SECOND") {
        compute_delay(lines, "SLOMORE_LINES_PER_SECOND")
    } else {
        default_delay()
    };

    // Determine how many lines to show immediately before applying the delay
    let initial_lines = if let Some(initial_lines) = matches.get_one::<usize>("initial_lines") {
        *initial_lines
    } else if let Some(initial_lines) = parse_env_usize("SLOMORE_INITIAL_LINES") {
        initial_lines
    } else {
        0
    };

    // Read from stdin and output with delay
    let stdin = io::stdin();
    let handle = stdin.lock();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    for (index, line) in handle.lines().enumerate() {
        if let Ok(line) = line {
            writeln!(out, "{line}").expect("write failed");
            if index + 1 > initial_lines {
                thread::sleep(delay);
            }
        } else {
            eprintln!("Error reading line");
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_values_must_be_finite_and_greater_than_zero() {
        assert!(is_positive(1.0));
        assert!(is_positive(0.1));
        assert!(!is_positive(0.0));
        assert!(!is_positive(-1.0));
        assert!(!is_positive(f64::NAN));
        assert!(!is_positive(f64::INFINITY));
    }

    #[test]
    fn default_delay_is_ten_lines_per_second() {
        assert_eq!(default_delay(), Duration::from_millis(100));
    }

    #[test]
    fn compute_delay_uses_lines_per_second() {
        assert_eq!(
            compute_delay(2.0, "lines-per-second"),
            Duration::from_millis(500)
        );
    }
}
