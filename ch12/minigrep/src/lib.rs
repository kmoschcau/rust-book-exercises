use std::error::Error;
use std::fs;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String], case_sensitive: bool) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod config_new {
    use super::*;

    #[test]
    fn one_argument_return_err() {
        let vec = vec!["minigrep".to_string()];

        assert_eq!(Err("not enough arguments"), Config::new(&vec));
    }

    #[test]
    fn two_arguments_return_err() {
        let vec = vec!["minigrep".to_string(), "foo".to_string()];

        assert_eq!(Err("not enough arguments"), Config::new(&vec));
    }

    #[test]
    fn three_arguments_return_ok_with_config() {
        let vec = vec!["minigrep".to_string(), "foo".to_string(), "bar".to_string()];

        assert_eq!(Ok(Config { query: "foo".to_string(), filename: "bar".to_string() }),
                   Config::new(&vec));
    }
}

#[cfg(test)]
mod search {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}

#[cfg(test)]
mod search_case_insensitive {
    use super::*;

    #[test]
    fn one_result() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
