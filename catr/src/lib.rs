use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut prev_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if config.number_lines {
                         println!("{:6}\t{line}", line_num + 1);
                    }
                    else if config.number_nonblank_lines {
                        if line.is_empty(){
                            println!();
                        } else {
                            prev_num += 1;
                            println!("{prev_num:6}\t{line}");
                        }
                    }
                    else {
                        println!("{line}");
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("SY")
        .about("Rust cat shakyo")
        .arg(
            Arg::with_name("files")     
            .value_name("FILE")
            .help("input file")
            .default_value("-")
            .multiple(true)
         )
        .arg(
            Arg::with_name("number")     
            .short("n")
            .long("number")
            .help("number all lines")
            .takes_value(false)
            .conflicts_with("number_non_blank"),
         )
        .arg(
            Arg::with_name("number_non_blank")     
            .short("b")
            .long("number-nonblank")
            .help("number non blank lines")
            .takes_value(false),
         )
        .get_matches();


    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_non_blank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
