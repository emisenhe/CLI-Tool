use log::{error, info, LevelFilter};
use std::fs::File;
use std::io::{BufReader, Read};
use std::io::Write;

let env_path = "https://github.com/PurdueSoftEng/CLI-Tool/blob/main/tool/.env.txt";
let env_file = File::open(env_path).unwrap(); 
let mut reader = BufReader::new(env_file);
let verbosity = reader.line().filter(|line| line.unwrap().contains("LOG_LEVEL"));
let log_level: i32 = verbosity.split_whitespace()
  .filter(|word| word.parse::<i32>().is_ok())
  .map(|word| word.trim().parse::<i32>().unwrap())
  .next()
  .unwrap();

let mut log_file = OpenOptions::new()
  .create(true)
  .append(true)
  .open("log_file.txt")
  .unwrap()

log::set_boxed_logger(Box::new(WriteLogger::new(log_level, log::LogConfig::default(), log_file))).unwrap();
log::set_max_level(log_level);