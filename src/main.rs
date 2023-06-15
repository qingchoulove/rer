use std::{fs, path::Path};
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};

use clap::{Parser, ValueEnum};
use regex::Regex;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Config {
    #[arg(long)]
    path: String,
    #[arg(long)]
    regex: String,
    #[arg(long)]
    name: String,
    #[arg(long)]
    year: u32,
    #[arg(long, value_enum)]
    source: Option<Source>,
    #[arg(long, value_enum)]
    clarity: Option<Clarity>,
    #[arg(long, value_enum)]
    encode: Option<Encode>,
}

fn main() {
    let cfg = Config::parse();
    let path = &cfg.path.clone();
    let rer = Rer::new(cfg);

    let path = Path::new(path);
    for entry in fs::read_dir(path).expect("read fail") {
        if let Ok(entry) = entry {
            let file = entry.path();
            let filename = file.to_str().unwrap();
            let resource = rer.parse(filename);
            if resource.is_some() {
                let ext = file.extension().unwrap_or(OsStr::new("mp4")).to_str().unwrap();
                let new_filename = format!("{}.{}", resource.unwrap().to_string(), ext);
                fs::rename(filename, new_filename).unwrap();
            }
        }
    }
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
        let season = caps.name("season").map_or(1, |m| m.as_str().parse::<u8>().unwrap());
        let ep = caps.name("ep").map_or(1, |m| m.as_str().parse::<u8>().unwrap());
        let name = &config.name;
        let year = config.year;
        let source = config.source.unwrap_or(Source::WEB_DL);
        let encode = config.encode.unwrap_or(Encode::H264);
        let clarity = config.clarity.unwrap_or(Clarity::C1080p);

        Some(Resource {
            name: name.to_string(),
            year,
            season,
            ep,
            source,
            encode,
            clarity,
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
