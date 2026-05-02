use crossbeam_channel::Sender;
use evdev::{Device, EventSummary, KeyCode};
use std::collections::HashMap;
use tokio::task;

/// Spawn one async listener per attached keyboard device. Each listener
/// reads evdev key events and forwards mapped scale indices into the
/// shared on/off channels.
///
/// Returns a JoinHandle that completes only if no keyboards could be
/// opened. Otherwise it lives forever (one underlying task per device).
pub fn create_keyboard_listener(
    on_chan: Sender<u16>,
    off_chan: Sender<u16>,
) -> task::JoinHandle<()> {
    task::spawn(async move {
	let mut spawned = 0usize;
	for (path, dev) in evdev::enumerate() {
	    if !looks_like_keyboard(&dev) {
		continue;
	    }
	    let on_c = on_chan.clone();
	    let off_c = off_chan.clone();
	    let path_str = path.display().to_string();
	    match dev.into_event_stream() {
		Ok(stream) => {
		    spawned += 1;
		    task::spawn(async move {
			run_device(stream, on_c, off_c, path_str).await;
		    });
		}
		Err(e) => {
		    eprintln!("qwertysynth: failed to open {} as event stream: {e}", path_str);
		}
	    }
	}
	if spawned == 0 {
	    eprintln!("\nno readable keyboard devices found.");
	    eprintln!("try: sudo usermod -aG input $USER  (then log out and back in)");
	}
    })
}

fn looks_like_keyboard(dev: &Device) -> bool {
    match dev.supported_keys() {
	Some(keys) => keys.contains(KeyCode::KEY_Q) && keys.contains(KeyCode::KEY_Z),
	None => false,
    }
}

async fn run_device(
    mut stream: evdev::EventStream,
    on_chan: Sender<u16>,
    off_chan: Sender<u16>,
    path: String,
) {
    loop {
	match stream.next_event().await {
	    Ok(ev) => {
		if let EventSummary::Key(_, code, value) = ev.destructure() {
		    // value: 0 = release, 1 = press, 2 = autorepeat (ignore)
		    let kernel_code = code.code() as u16;
		    if let Some(idx) = key_map_convert(kernel_code) {
			match value {
			    1 => { let _ = on_chan.send(idx); }
			    0 => { let _ = off_chan.send(idx); }
			    _ => {}
			}
		    }
		}
	    }
	    Err(e) => {
		eprintln!("qwertysynth: read error on {path}: {e}");
		return;
	    }
	}
    }
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
	(11,39u16),
    ]);
    keymap.get(&val).copied()
}

