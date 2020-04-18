
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argumenrs");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn given_two_arguments(){
        let mock_args=[String::from("test0"),String::from("test1"),String::from("test2")];
        assert_eq!(String::from("test1"),Config::new(&mock_args).unwrap().query);
        assert_eq!(String::from("test2"),Config::new(&mock_args).unwrap().filename);
    }
} 