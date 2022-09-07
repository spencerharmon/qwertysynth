
//! Sine wave generator with frequency configuration exposed through standard
//! input.

use std::{thread, time};
use crossbeam_channel::*;
mod equal_temperment;
mod wave_table;
mod scale;
mod instrument;
mod output;
mod sound_test;
mod polysynth;

fn main() {
    
    let output = output::Output::new();
    output::Output::jack_output();
    println!("outside function");
//    in_L.send(0.1);

//    output::Output::gen_sound(buffer_L_tx, buffer_R_tx);
    
    wait_async_loop();

}

pub fn wait_async_loop() {
    let ten_millis = time::Duration::from_millis(10);
    let now = time::Instant::now();
    loop {
        thread::sleep(ten_millis);
    }
}
