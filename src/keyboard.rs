use keyboard_query;

use tokio::task;
use crossbeam_channel::*;
use keyboard_query::{DeviceQuery, DeviceState};
use std::collections::HashMap;

pub fn create_keyboard_listener(on_chan: Sender<u16>, off_chan: Sender<u16>) -> task::JoinHandle<()>{
    let fut = task::spawn(async move {
        let device_state = DeviceState::new();
        let mut prev_keys = vec![];
        loop {
            let keys = device_state.get_keys();
            if keys != prev_keys {
		for k in &prev_keys {
		    if !keys.contains(k) {
			//note off event
			let c = key_map_convert(*k);
			if let Some(x) = c {
			    println!("{}",x);
                            off_chan.send(x);
			}
		    }
		}
		for k in &keys {
		    if !prev_keys.contains(k) {
			//note on event
			let c = key_map_convert(*k);
			if let Some(x) = c {
			    println!("{}",x);
                            on_chan.send(x);
			}
		    }
		}
            }
            prev_keys = keys;
        }
    });
    fut
}

pub fn key_map_convert(val: u16) -> Option<u16> {
    let keymap = HashMap::from([
        (44,1u16),
        (30,2u16),
        (16,3u16),
        (2,4u16),
        (45,5u16),
        (31,6u16),
        (17,7u16),
        (3,8u16),
        (46,9u16),
        (32,10u16),
        (18,11u16),
        (4,12u16),
        (47,13u16),
        (33,14u16),
        (19,15u16),
        (5,16u16),
        (48,17u16),
        (34,18u16),
        (20,19u16),
        (6,20u16),
        (49,21u16),
        (35,22u16),
        (21,23u16),
        (7,24u16),
        (50,25u16),
        (36,26u16),
        (22,27u16),
        (8,28u16),
        (51,29u16),
        (37,30u16),
        (23,31u16),
        (9,32u16),
        (52,33u16),
        (38,34u16),
        (24,35u16),
        (10,36u16),
        (53,37u16),
        (39,38u16),
        (25,39u16),
        (11,40u16)
    ]);
    let opt_ref = keymap.get(&val);
    if let Some(v) = opt_ref {
	Some(*v)
    } else {
	None
    }
}

