use tokio;
use crossbeam_channel::*;
mod equal_temperment;
mod wave_table;
mod scale;
mod instrument;
mod output;
mod sound_test;
mod polysynth;
mod keyboard;
mod voice;


use clap::Parser;
    
#[derive(Parser)]
struct Cli {
    #[clap(short='b', long="base_freq", default_value_t=wave_table::DEFAULT_BASE_FREQUENCY)]
    base_freq: f32,
    #[clap(short='s', long="subdivisions", default_value_t=equal_temperment::DEFAULT_SUBDIVISIONS)]
    subdivisions: u8,
    #[clap(arg_enum, default_value="sine")]
    voice: voice::VoiceList,
}


#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let output = output::Output::new();
    output::Output::jack_output(args.base_freq, args.subdivisions).await;
}
