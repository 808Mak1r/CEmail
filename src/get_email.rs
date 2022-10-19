use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use tracing::{error, info};

pub fn get_email_list(filename: String) -> Vec<String> {
    let mut email_list: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        info!("开始获取邮箱列表");
        for line in lines {
            if let Ok(email) = line {
                email_list.push(email);
            }
        }
        info!("邮箱列表获取完毕");
    }

    match email_list.len() {
        0 => {
            error!("没有可用的邮箱");
            std::process::exit(0);
        }
        _ => {
            info!("共有 {} 个邮箱", email_list.len());
            return email_list;
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
