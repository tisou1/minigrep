use std::env;
use std::process;

use minigrep::Config;


fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // 读取的第一个参数数为可执行路径名, 后面才是我们输入的
    // println!("success");

    // 存储读取到的参数, 直接穿给build迭代器
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    // 文件读取
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
   
}
