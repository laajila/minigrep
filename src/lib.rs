use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive:bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argumenrs");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive})
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results= if config.case_sensitive {
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
        if line.contains(&query) {
            results.push(line)
        }
    }
    results
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query=query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn given_two_arguments() {
        let mock_args = [
            String::from("test0"),
            String::from("test1"),
            String::from("test2"),
        ];
        assert_eq!(
            String::from("test1"),
            Config::new(&mock_args).unwrap().query
        );
        assert_eq!(
            String::from("test2"),
            Config::new(&mock_args).unwrap().filename
        );
    }

    #[test]
    fn file_exist() {
        assert!(run(Config {
            query: "test".to_string(),
            filename: "poem.txt".to_string(),
            case_sensitive:false
        })
        .is_ok());
        assert!(run(Config {
            query: "test".to_string(),
            filename: "poemo.txt".to_string(),
            case_sensitive:false
        })
        .is_err());
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insenstive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
pick three.Trust";
        assert_eq!(vec!["Rust:","pick three.Trust"], search_case_insensitive(query, contents))
    }
}
