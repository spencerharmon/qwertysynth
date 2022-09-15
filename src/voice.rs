pub mod sine;

use std::str::FromStr;
use std::io;
use crate::voice::sine::Sine;
use clap::ArgEnum;

#[derive(Clone, ArgEnum)]
pub enum VoiceList {
    Sine,
}

impl FromStr for VoiceList {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, io::Error> {
	match s.to_ascii_lowercase().as_str() {
	    "sine" => Ok(VoiceList::Sine),
	    _ => return Err(std::io::Error::new(io::ErrorKind::Other, "invalid voice")),
	}
    }
}
