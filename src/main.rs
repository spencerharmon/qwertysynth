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

#[tokio::main]
async fn main() {
    let output = output::Output::new();
    output::Output::jack_output().await;

}
