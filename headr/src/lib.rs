use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

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
        .about("rust head")
        .arg(
            Arg::with_name("files")     
            .value_name("FILE")
            .help("Input file(s)")
            .default_value("-")
            .multiple(true)
         )
        .arg(
            Arg::with_name("lines")     
            .value_name("LINES")
            .short("n")
            .long("lines")
            .help("Number of lines")
            .default_value("10")
         )
        .arg(
            Arg::with_name("bytes")     
            .value_name("BYTES")
            .short("c")
            .long("bytes")
            .help("Number of bytes")
            .conflicts_with("lines")
            .takes_value(true)
         )
        .get_matches();


    let lines = matches.
        value_of("lines")
        .map(|v| parse_positive_int(v, "'--lines <LINES>'"))
        .transpose()
        .map_err(|e| format!("error: {} bar", e))?;

    let bytes = matches.
        value_of("bytes")
        .map(|v| parse_positive_int(v, "'--bytes <BYTES>'"))
        .transpose()
        .map_err(|e| format!("{}", e))?;


    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 {"\n"} else {""},
                        filename
                    );
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                        print!(
                            "{}",
                            String::from_utf8_lossy(&buffer[..bytes_read])
                    );
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
                //for line in file.lines().take(config.lines) {
                //    println!("{}", line?);
                //}
            }
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str, line_or_byte: &str) -> MyResult<usize> {
    match val.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        Ok(_) => Err(From::from(format!(
            "invalid value '{}' (must be > 0)",
            val
        ))),
        Err(e) => Err(From::from(format!(
            "invalid value '{}' for {}: {}",
            val, line_or_byte,  e
        ))),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3", "hoge");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo", "fuga");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "invalid value 'foo' for fuga: invalid digit found in string".to_string());
    let res = parse_positive_int("0", "piyo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "invalid value '0' (must be > 0)".to_string());
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
