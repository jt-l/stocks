use std::error::Error;
use std::env;
use std::process;

mod db;
mod api;
mod formatter;

use db::Queries;


// valid commands
#[derive(Clone)]
pub enum Command {
    InsertStock {arg: String},
    RemoveStock {arg: String},
    GetStocks,
}

trait FromStr {
    fn from_str(args: &[String]) -> Result<Command, (&'static str)>;
}

// FromStr is used to parse command line arg into enum
impl FromStr for Command {

    fn from_str(args: &[String]) -> Result<Command, (&'static str)> {
       let command = &args[1];

        match command.as_ref() {
            "add" => {
                if args.len() < 3 { return Err("Not enough arguments")};
                Ok(Command::InsertStock {arg: args[2].clone()})
            },
            "rm" => {
                if args.len() < 3 { return Err("Not enough arguments")};
                Ok(Command::RemoveStock {arg: args[2].clone()})
            },
            "ls" => Ok(Command::GetStocks),
            _ => Err("Invalid command"),
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub command: Command, 
    pub api_key: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        let command = Command::from_str(args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);   
            process::exit(1);
        });

        let api_key = env::var("WORLD_TRADING_DATA_API_KEY").unwrap_or_else(|err| {
            eprintln!("WORLD_TRADING_DATA_API_KEY is not set: {}", err);   
            process::exit(1);
        });

        Ok(Config {command, api_key})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // create tables if they do not exist
    db::execute(config.clone(), Queries::CreateTables)?;

    // execute command
    match config.command {
        Command::InsertStock{arg: _ }   => { db::execute(config, Queries::InsertStock)?; },
        Command::RemoveStock{arg: _ }   => { db::execute(config, Queries::RemoveStock)?; },
        Command::GetStocks              => { db::execute(config, Queries::GetStocks)?; },
    }

    Ok(())
}
