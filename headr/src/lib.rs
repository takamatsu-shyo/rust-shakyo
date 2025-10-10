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

    let files: Vec<String> = matches.values_of("files").unwrap_or_default().map(|v| v.to_string()).collect();
    //let lines: usize = matches.value_of("lines").unwrap().parse().unwrap_or(10); // Default to 10 if parsing fails
    //let bytes: Option<usize> = matches.value_of("bytes").map(|v| v.parse().unwrap());


    let lines_opt: Option<usize> = match matches.value_of("lines") {
        Some(v) => Some(parse_positive_int(v).map_err(|_| {
            format!(
                "invalid value '{}' for '--lines <LINES>': invalid digit found in string",
                v
            )
        })?),
        None => None,
    };


    let bytes: Option<usize> = match matches.value_of("bytes") {
        Some(v) => Some(parse_positive_int(v).map_err(|_| {
            format!(
                "invalid value '{}' for '--bytes <BYTES>': invalid digit found in string",
                v
            )
        })?),
        None => None,
    };


    //let lines = matches.value_of::<usize>("lines").ok();
    //let bytes = matches.value_of::<usize>("bytes").ok();

    let lines = lines_opt.unwrap();

    Ok(Config {
        files,
        lines,
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

//fn parse_positive_int(val: &str) -> MyResult<usize> {
//    match val.parse() {
//        Ok (n) if n > 0 => Ok(n),
//        _ => Err(From::from(val)),
//    }
//
//}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        Ok(_) => Err(From::from(format!(
            "invalid value '{}' (must be > 0)",
            val
        ))),
        Err(e) => Err(From::from(format!(
            "invalid value '{}': {}",
            val, e
        ))),
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
