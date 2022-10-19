use clap::Parser;
use time::macros::format_description;
use tracing::{error, info, Level};
use tracing_subscriber::{fmt::time::LocalTime, FmtSubscriber};

use cemail::cli::Args;
use cemail::check::check;
use cemail::get_email::get_email_list;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let timer = LocalTime::new(format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"));

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_timer(timer)
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    if args.email.is_none() && args.file.is_none() {
        error!("没有可用的邮箱");
        std::process::exit(0);
    } else {
        if args.email.is_none() {
            let emails = get_email_list(args.file.unwrap());
            check(emails, args.output).await.unwrap();
        } else {
            info!("开始读取邮箱");
            let email = args.email;
            info!("邮箱读取完毕");
            check(email.unwrap(), args.output).await.unwrap();
        }
    }
}
