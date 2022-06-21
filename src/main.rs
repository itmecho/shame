mod sha256;

use std::{
    env::args,
    error::Error,
    fs,
    io::{self, Read},
    process::exit,
};

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = parse_args(args().skip(1));
    if cfg.unknown_args.len() > 0 {
        for arg in cfg.unknown_args.iter() {
            eprintln!("unknown argument: {}", arg);
        }
        exit(1)
    }

    let data = match cfg.filename {
        Some(fname) => Vec::from(fs::read(fname)?),
        None => {
            let mut buf = vec![];
            io::stdin().read_to_end(&mut buf)?;
            buf
        }
    };

    match cfg.mode {
        Mode::SHA256 => sha256::generate_hash(&data),
    };

    Ok(())
}

enum Mode {
    SHA256,
}

impl Default for Mode {
    fn default() -> Self {
        Self::SHA256
    }
}

#[derive(Default)]
struct Config {
    pub filename: Option<String>,
    pub mode: Mode,
    pub unknown_args: Vec<String>,
}

fn parse_args<T>(args: T) -> Config
where
    T: Iterator<Item = String>,
{
    let mut cfg = Config::default();

    for arg in args {
        match arg.as_str() {
            "--sha256" => cfg.mode = Mode::SHA256,
            s if !s.starts_with("--") => cfg.filename = Some(arg),
            _ => cfg.unknown_args.push(arg),
        }
    }

    cfg
}
