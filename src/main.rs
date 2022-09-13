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
//mod cli;

use clap::Parser;
    
#[derive(Parser)]
struct Cli {
    #[clap(short='b', long="base_freq", default_value_t=wave_table::DEFAULT_BASE_FREQUENCY)]
    base_freq: f32,
}


#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let output = output::Output::new();
    output::Output::jack_output(args.base_freq).await;
}
