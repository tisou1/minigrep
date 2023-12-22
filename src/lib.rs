
use std::error::Error;
use std::{fs, env};

 pub struct Config {
  query: String,
  file_path: String, 
  pub ignore_case: bool, // 是否要开启大小写敏感
}

impl Config {
  pub fn build(
    mut args: impl Iterator<Item = String>,
  ) -> Result<Config, &'static str> {
      // if args.len() < 3 {
      //     // panic!("not enough arguments");
      //     return Err("not enough arguments");
      // }
      args.next();

      let query = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a query string"),
      };
      let file_path = match args.next() {
          Some(arg) => arg,
          None => return  Err("Didn't get a file path"),
      };

      // var读取环境变量, is_ok是Result提供的, 有值返回true没值返回false
      // let ignore_case = env::var("IGNORE_CASE").is_ok();

      // map_or_else 是Result上的方法, 接受两个闭包参数, 第一个处理Err情况 第二个处理Ok情况
      let ignore_case =  env::var("IGNORE_CASE").map_or_else(|_| {
        args
          .any(|arg| arg.to_lowercase() == "-i" || arg.to_lowercase() == "--ignore-case")
      },
      |env_value| env_value == "0" || env_value.to_lowercase() == "false",
      );
     Ok( Config{query, file_path, ignore_case})
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents =  fs::read_to_string(config.file_path)?;
  

  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &contents)
  } else {
    search(&config.query, &contents)
  };

  for line in results {
    println!("{line}");
  }

  Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//  let mut results = Vec::new();
//  for line in contents.lines() {
//   if line.contains(query) {
//     results.push(line.trim());
//   }
//  }
//  results

contents
  .lines()
  .filter(|line| line.contains(query))
  .collect()
}

pub fn search_case_insensitive<'a>(
  query: &str,
  contents: &'a str
) -> Vec<&'a str> {
  let query = query.to_lowercase();
  // let mut results = Vec::new();

  // for line in contents.lines() {
  //   if line.to_lowercase().contains(&query) {
  //     results.push(line);
  //   }
  // }
  // results

  contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()
}

#[cfg(test)]
mod tests {
  // 导入付模块的所有公共项
  use super::*;

  // 注定失败的
  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
    Rust:
    Safe, fast, productive
    Pick three.";

    assert_eq!(vec!["Safe, fast, productive"], search(query, contents));
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