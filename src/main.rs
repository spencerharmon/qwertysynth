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


use crate::voice::Voice;

use clap::Parser;
#[derive(Parser)]
struct Cli {
    #[clap(short='b', long="base_freq", default_value_t=wave_table::DEFAULT_BASE_FREQUENCY)]
    base_freq: f32,
    #[clap(short='s', long="subdivisions", default_value_t=equal_temperment::DEFAULT_SUBDIVISIONS)]
    subdivisions: u8,
    #[clap(arg_enum, long="voice", default_value="sine")]
    voice: voice::VoiceList,
}


#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let output = output::Output::new();


    let et = equal_temperment::EqualTemperment::new(
        args.base_freq,
	args.subdivisions,
	equal_temperment::DEFAULT_MULTIPLIER,
    );

    let scale = et.generate_scale();
    

    let mut scale_wave_tables: Vec<wave_table::WaveTable> = Vec::new();
            

    for f in scale.get_frequencies_vector() {
	let wt = args.voice.get_wavetable(
	    f,
	    wave_table::DEFAULT_SAMPLE_RATE,
	    wave_table::DEFAULT_AMPLITUDE,
	    wave_table::DEFAULT_PHASE
	);

	scale_wave_tables.push(wt);
    }
    let mut instrument = instrument::Instrument::new(scale_wave_tables);
    
    output::Output::jack_output(args.base_freq, args.subdivisions, instrument).await;
}
