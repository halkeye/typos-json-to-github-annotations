use clap::Parser;
use serde::Deserialize;
use std::fmt;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Deserialize)]
pub struct Typo {
    path: String,
    line_num: i32,
    byte_offset: i32,
    typo: String,
    corrections: Vec<String>,
}

impl fmt::Display for Typo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "::error file={},line={},col={}::{} should be {}",
            self.path,
            self.line_num,
            self.byte_offset,
            self.typo,
            itertools::join(self.corrections.iter().map(|s| format!("`{}`", s)), ", "),
        )
    }
}

#[derive(Deserialize)]
pub struct BinaryFile {
    // path: String
}


#[derive(Deserialize)]
#[serde(tag = "type")]
enum Message {
    #[serde(rename = "binary_file")]
    BinaryFile(BinaryFile),
    #[serde(rename = "typo")]
    Typo(Typo)
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let input: Box<dyn std::io::Read + 'static> = if args.path.as_os_str() == "-" {
        Box::new(std::io::stdin())
    } else {
        match std::fs::File::open(&args.path) {
            Ok(file) => Box::new(file),
            Err(err) => {
                let msg = format!("{}: {}", args.path.display(), err);
                return Err(Error::from(msg));
            }
        }
    };

    let reader = BufReader::new(input);

    for line in reader.lines() {
        let unwrapped_line = line.unwrap();
        let msg: Message = serde_json::from_str(&unwrapped_line)?;
        match msg {
            Message::BinaryFile(_) => {}
            Message::Typo(typo) => println!("{}", typo),
        }
    }

    Ok(())
}

/*
*/
