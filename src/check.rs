use check_if_email_exists::Reachable::{Invalid, Risky, Safe, Unknown};
use check_if_email_exists::{check_email, CheckEmailInput, CheckEmailOutput};
use serde::Serialize;
use std::fs::OpenOptions;
use std::path::PathBuf;
use tracing::{error, info};

pub async fn check(
    email_list: Vec<String>,
    output: PathBuf,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let to_email = email_list;

    let input = CheckEmailInput::new(to_email.to_vec());

    let result = check_email(&input).await;

    output_file(output_info(&result).unwrap(), output).unwrap();

    Ok(())
}

fn _output_result(results: &Vec<CheckEmailOutput>) {
    match serde_json::to_string_pretty(&results) {
        Ok(output) => {
            if output != "[]" {
                info!("{}", output);
            } else {
                error!("没有可用的邮箱");
            }
        }
        Err(err) => {
            error!("{}", err);
        }
    };
}

#[derive(Debug, Serialize)]
struct OutputInfo<'a> {
    safe: Vec<String>,
    risky: Vec<String>,
    invalid: Vec<String>,
    unknown: Vec<String>,
    ce_output: &'a Vec<CheckEmailOutput>,
}

fn output_info(results: &Vec<CheckEmailOutput>) -> Result<OutputInfo, Box<dyn std::error::Error>> {
    let mut out_file = OutputInfo {
        safe: Vec::new(),
        risky: Vec::new(),
        invalid: Vec::new(),
        unknown: Vec::new(),
        ce_output: results,
    };

    // let out_f = OutputInfo{
    //     safe: results
    //         .iter()
    //         .filter(|x| x.is_reachable == Safe)
    //         .map(|x| x.input.clone())
    //         .collect(),
    //     risky: results
    //         .iter()
    //         .filter(|x| x.is_reachable == Risky)
    //         .map(|x| x.input.clone())
    //         .collect(),
    //     invalid: results
    //         .iter()
    //         .filter(|x| x.is_reachable == Invalid)
    //         .map(|x| x.input.clone())
    //         .collect(),
    //     unknown: results
    //         .iter()
    //         .filter(|x| x.is_reachable == Unknown)
    //         .map(|x| x.input.clone())
    //         .collect(),
    // };

    for result in results.iter() {
        info!("邮箱: {} 状态: {:?}", result.input, result.is_reachable);
        match &result.is_reachable {
            Safe => out_file.safe.push(result.input.clone()),
            Risky => out_file.risky.push(result.input.clone()),
            Invalid => out_file.invalid.push(result.input.clone()),
            Unknown => out_file.unknown.push(result.input.clone()),
        }
    }

    Ok(out_file)
}

fn output_file(out_file: OutputInfo, file_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut f_path = PathBuf::from(file_path);

    if f_path.exists() {
        for i in 1..101 {
            f_path = PathBuf::from(format!("output{}.json", i));
            match f_path.exists() {
                false => break,
                true => (),
            };
        }
    };

    // println!("{}", f_path.display());

    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(&f_path)?;

    // match f.write_all(format!("{:#?}", out_file).as_bytes()) {
    //     Ok(_) => (),
    //     Err(err) => error!("{}", err),
    // };
    // match f.write_all(format!("{:#?}", serde_json::to_string(results)).as_bytes()) {
    //     Ok(_) => (),
    //     Err(err) => error!("{}", err),
    // };

    serde_json::to_writer_pretty(f, &out_file)?;

    Ok(info!("输出到文件: {}", f_path.display()))
}
