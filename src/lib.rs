use std::error::Error;
use std::fs;


pub struct Config{
    query:String,
    file_path:String
}

pub fn run(config:Config)-> Result<(),Box<dyn Error>>{
    println!("Searching for the query {}", config.query);
    println!("In the file {}",config.file_path );
    let contents = fs::read_to_string(config.file_path)?;
    println!("With texts:\n{}", contents);
    Ok(())
}



impl Config{
   pub fn build(args: &[String]) -> Result<Config,&'static str>{
        if args.len() < 3  {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config{query, file_path})

    }
}
