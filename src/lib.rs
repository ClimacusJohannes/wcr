use std::error::Error;
use clap::Parser;
use crate::styles::get_styles;
use colored::Colorize;

pub mod styles;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug, Clone)]
#[command(name = "wcr", author = "Izak Hudnik Zajec <hudnik.izak@gmail.com>", version = "0.1.0", about = "wc implemented in Rust", styles=get_styles())]
pub struct Cli {
    #[arg(
        help = "Input file(s)",
        default_values_t = vec!["-".to_owned()],
        )]
    files: Vec<String>,

    #[arg(
        short = 'c', long,
        default_value_t = false,
        help = "Show byte count",
        )]
    bytes: bool,

    #[arg(
        short = 'm', long,
        default_value_t = false,
        help = "Show characted count",
        )]
    chars: bool,

    #[arg(
        short, long,
        default_value_t = false,
        help = "Show line count",
        )]
    lines: bool,

    #[arg(
        short, long,
        default_value_t = false,
        help = "Show word count",
        )]
    words: bool,
}


pub fn cli() -> MyResult<Cli> {
    let mut cli_temp : Cli = Parser::parse();

    if [cli_temp.lines, cli_temp.words, cli_temp.chars, cli_temp.bytes].iter().all(|v| v == &false) {
        cli_temp.lines = true;
        cli_temp.words = true;
        cli_temp.bytes = true;
    }

    let cli = cli_temp.clone();

    Ok(cli)
}

pub fn run(cli : Cli) -> MyResult<()> {
    println!("{:#?}", cli);
    Ok(())
}
