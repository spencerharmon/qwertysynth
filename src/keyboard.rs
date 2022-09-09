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
                            off_chan.send(x);
			}
		    }
		}
		for k in &keys {
		    if !prev_keys.contains(k) {
			//note on event
			let c = key_map_convert(*k);
			if let Some(x) = c {
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
        (44,0u16),
        (30,1u16),
        (16,2u16),
        (2,3u16),
        (45,4u16),
        (31,5u16),
        (17,6u16),
        (3,7u16),
        (46,8u16),
        (32,9u16),
        (18,10u16),
        (4,11u16),
        (47,12u16),
        (33,13u16),
        (19,14u16),
        (5,15u16),
        (48,16u16),
        (34,17u16),
        (20,18u16),
        (6,19u16),
        (49,20u16),
        (35,21u16),
        (21,22u16),
        (7,23u16),
        (50,24u16),
        (36,25u16),
        (22,26u16),
        (8,27u16),
        (51,28u16),
        (37,29u16),
        (23,30u16),
        (9,31u16),
        (52,32u16),
        (38,33u16),
        (24,34u16),
        (10,35u16),
        (53,36u16),
        (39,37u16),
        (25,38u16),
        (11,39u16)
    ]);
    let opt_ref = keymap.get(&val);
    if let Some(v) = opt_ref {
	Some(*v)
    } else {
	None
    }
}

