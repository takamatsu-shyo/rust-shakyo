use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("SY")
        .about("head by Rust")
        .arg(
            Arg::with_name("files")
            .value_name("FILE")
            .help("input file")
            .default_value("-")
            .multiple(true)
        )
        .arg(
            Arg::with_name("lines")
            .short("n")
            .long("lines")
            .value_name("LINES")
            .help("Number of lines to show")
            .default_value("10")
            .conflicts_with("bytes")
        )
        .arg(
            Arg::with_name("bytes")
           .short("c") 
           .long("bytes")
           .value_name("BYTES")
           .help("Number of bytes to show")
           .takes_value(true)
           .conflicts_with("lines")
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();

    let lines = matches
        .value_of("lines")
        .unwrap()
        .parse::<usize>()
        .map_err(|e| format!("invalid value '' for --lines <LINES>. {e}"))?;


    let bytes = matches
        .value_of("bytes")
        .map(|v| v.parse::<usize>())
        .transpose()
        .map_err(|e| format!("Invalid value for --bytes. {e}"))?;



    Ok(Config {
        files, lines, bytes
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

// Ref: https://doc.rust-lang.org/book/ch06-02-match.html
fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse(){
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

