use std::{env, fs, path::Path};
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};

use anyhow::Result;
use clap::{Parser, ValueEnum};
use regex::Regex;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Config {
    #[arg(long)]
    path: Option<String>,
    #[arg(long)]
    regex: String,
    #[arg(long)]
    name: Option<String>,
    #[arg(long)]
    year: Option<u32>,
    #[arg(long)]
    season: Option<u8>,
    #[arg(long, value_enum)]
    source: Option<Source>,
    #[arg(long, value_enum)]
    clarity: Option<Clarity>,
    #[arg(long, value_enum)]
    encode: Option<Encode>,
}

fn main() -> Result<()> {
    let current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    let cfg = Config::parse();
    let path = &cfg.path.clone().unwrap_or(current_dir);
    let rer = Rer::new(cfg);

    let path = Path::new(path);
    for entry in fs::read_dir(path).expect("read fail") {
        if let Ok(entry) = entry {
            let file = entry.path();
            let filename = file.to_str().unwrap();
            let resource = rer.parse(filename);
            if let Some(r) = resource {
                let ext = file.extension().unwrap_or(OsStr::new("mp4")).to_str().unwrap();
                let new_filename = format!("{}.{}", r.to_string(), ext);
                fs::rename(filename, new_filename).unwrap();
            }
        }
    }
    Ok(())
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Source {
    #[allow(non_camel_case_types)]
    WEB_DL,
    HDTV,
    DVD,
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Source::WEB_DL => write!(f, "WEB_DL"),
            Source::HDTV => write!(f, "HDTV"),
            Source::DVD => write!(f, "DVD"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Encode {
    H264,
    H265,
    HEVC,
}

impl Display for Encode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Encode::H264 => write!(f, "H264"),
            Encode::H265 => write!(f, "H265"),
            Encode::HEVC => write!(f, "HEVC"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Clarity {
    #[clap(name = "720p")]
    C720p,
    #[clap(name = "1080p")]
    C1080p,
    #[clap(name = "2k")]
    C2k,
    #[clap(name = "4k")]
    C4k,
}

impl Display for Clarity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Clarity::C720p => write!(f, "720p"),
            Clarity::C1080p => write!(f, "1080p"),
            Clarity::C2k => write!(f, "2k"),
            Clarity::C4k => write!(f, "4k"),
        }
    }
}

struct Rer {
    config: Config,
}

impl Rer {
    pub fn new(config: Config) -> Self {
        Rer {
            config
        }
    }

    fn parse(&self, filename: &str) -> Option<Resource> {
        let config = &self.config;
        let re = Regex::new(&config.regex).unwrap();
        let caps = re.captures(filename);
        if caps.is_none() {
            return None;
        }
        let caps = caps.unwrap();
        if caps.name("ep").is_none() {
            return None;
        }

        let default_name = config.name.clone().unwrap_or("".to_string());
        let default_season = config.season.unwrap_or(1);
        let default_year = config.year.unwrap_or(2023);
        let default_source = config.source.unwrap_or(Source::WEB_DL);
        let default_encode = config.encode.unwrap_or(Encode::H264);
        let default_clarity = config.clarity.unwrap_or(Clarity::C1080p);

        let season = caps.name("season").map_or(default_season, |m| m.as_str().parse::<u8>().unwrap());
        let ep = caps.name("ep").map_or(1, |m| m.as_str().parse::<u8>().unwrap());

        Some(Resource {
            name: default_name,
            year: default_year,
            season,
            ep,
            source: default_source,
            encode: default_encode,
            clarity: default_clarity,
        })
    }
}

#[derive(PartialEq)]
struct Resource {
    name: String,
    year: u32,
    season: u8,
    ep: u8,
    source: Source,
    encode: Encode,
    clarity: Clarity,
}


impl ToString for Resource {
    fn to_string(&self) -> String {
        format!("{}.{}.S{}E{}.{}.{}.{}",
                self.name,
                self.year,
                self.season,
                self.ep,
                self.source,
                self.clarity,
                self.encode)
    }
}
