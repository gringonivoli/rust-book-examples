use std::{env, error::Error, fs};

// probando cosas...
pub trait Config {
    fn query(&self) -> &str;
    fn file_path(&self) -> &str;
    fn ignore_case(&self) -> bool;
}

// probando cosas...
pub fn run(config: impl Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path())?;
    let result = if config.ignore_case() {
        search_case_insensitive(config.query(), &contents)
    } else {
        search(config.query(), &contents)
    };
    for line in result {
        println!("{line}");
    }
    Ok(())
}

pub struct DefaultConfig {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl DefaultConfig {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<DefaultConfig, &'static str> {
        args.next();
        Ok(DefaultConfig {
            query: match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string."),
            },
            file_path: match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file path."),
            },
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

// probando cosas...
impl Config for DefaultConfig {
    fn query(&self) -> &str {
        &self.query
    }

    fn file_path(&self) -> &str {
        &self.file_path
    }

    fn ignore_case(&self) -> bool {
        self.ignore_case
    }
}

// probando cosas...
pub struct AnotherConfig {
    args: Vec<String>,
    ignore_case: bool,
}

// probando cosas...
impl AnotherConfig {
    pub fn build(args: Vec<String>) -> Result<AnotherConfig, &'static str> {
        Ok(AnotherConfig {
            args,
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

// probando cosas...
impl Config for AnotherConfig {
    fn query(&self) -> &str {
        &self.args[1]
    }

    fn file_path(&self) -> &str {
        &self.args[2]
    }

    fn ignore_case(&self) -> bool {
        self.ignore_case
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lowercase_query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&lowercase_query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
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
