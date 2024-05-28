use std::{error::Error, fmt::format, fs::File, hash::Hash, io::{self, BufRead, BufReader}};
use clap::Parser;
use words_count::count;
use crate::styles::get_styles;

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

#[derive(Debug, PartialEq)]
struct FileInfo {
    num_lines : usize,
    num_words : usize,
    num_chars : usize,
    num_bytes : usize,
}

type ListInfo = (FileInfo, String);

pub fn cli() -> MyResult<Cli> {
    let mut cli_temp : Cli = Parser::parse();

    if [cli_temp.lines, cli_temp.words, cli_temp.chars, cli_temp.bytes].iter().all(|v| v == &false) {
        cli_temp.lines = true;
        cli_temp.words = true;
        cli_temp.bytes = true;
    }

    // cli should not be mutable when passed on.
    Ok(cli_temp.clone())
}


fn open(filename : &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run_count(mut file : impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_chars = 0;
    let mut num_bytes = 0;

    loop {
        let mut line = "".to_owned();
        match file.read_line(&mut line) {
            Ok(0) => {
                break;
            },
            Ok(count) => {
                num_lines += 1;
                num_chars += count;
                let result = words_count::count(&line);
                num_words += result.words;
                for byte in line.to_owned().bytes() {
                    num_bytes += 1;
                }
            },
            Err(e) => {
                eprintln!("Error while reading line: {:?}", e);
            }
        }
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_chars,
        num_bytes,
    })
}

fn display(info: Vec<ListInfo>, cli: Cli) {
    let mut list : Vec<Vec<String>>; 

    for d in info {
        println!("{:#?}", d);
    }
}



pub fn run(cli : Cli) -> MyResult<()> {
    println!("{:#?}", cli);
    let mut results : Vec<ListInfo> = vec![];
    for filename in &cli.files {
        match open(&filename)
            .and_then(|info| Ok(run_count(info))) {
                Ok(Ok(info)) => {
                    results.push((info, filename.clone()));
                },
                Ok(Err(err)) => {
                    let info = FileInfo {
                        num_bytes: 0,
                        num_chars: 0,
                        num_words: 0,
                        num_lines: 0,
                    };
                    results.push((info, format!("{}: {}", &filename, err.to_string())))
                },
                Err(err) => {
                    let info = FileInfo {
                        num_bytes: 0,
                        num_chars: 0,
                        num_words: 0,
                        num_lines: 0,
                    };
                    results.push((info, format!("{}: {}", &filename, err.to_string())))
                },
            }

    }

    display(results, cli);
    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::run_count;

    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = run_count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
