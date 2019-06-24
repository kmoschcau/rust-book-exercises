use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let case_sensitive = case_sensitive(&args);

    let config = Config::new(&args, case_sensitive).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

fn case_sensitive(args: &[String]) -> bool {
    if args.contains(&"--no-ignore-case".to_string()) {
        return true;
    }
    if args.contains(&"--ignore-case".to_string()) {
        return false;
    }
    env::var("CASE_INSENSITIVE").is_err()
}
