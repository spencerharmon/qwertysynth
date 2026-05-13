use crate::tuning::TuningSystem;

/// Stern-Brocot tuning system. Enumerates the first `num_notes`
/// positive rationals in (1, framing_interval] from a Stern-Brocot
/// tree restricted to that window, ordered by complexity (the order
/// in which the algorithm inserts each new mediant). Multiplies each
/// by `base_freq`.
pub struct SternBrocotScale {
    pub base_freq: f32,
    pub framing_interval: f32,
}

impl SternBrocotScale {
    pub fn new(base_freq: f32, framing_interval: f32) -> Self {
	Self {
	    base_freq,
	    framing_interval: framing_interval.max(1.001),
	}
    }
}

impl TuningSystem for SternBrocotScale {
    fn generate_scale(&self, num_notes: usize) -> Vec<f32> {
	// Insert mediants between adjacent fractions, bounded by
	// (1/1, framing_interval/1). Each pass through the list
	// inserts one mediant per existing adjacent pair, doubling
	// the number of intervals. Collect insertion order to give a
	// natural complexity ranking.
	let frame_num = self.framing_interval.round() as i64;
	let mut endpoints: Vec<(i64, i64)> = vec![(1, 1), (frame_num.max(2), 1)];
	let mut insertion_order: Vec<(i64, i64)> = Vec::new();

	while insertion_order.len() < num_notes {
	    let mut new_endpoints: Vec<(i64, i64)> = Vec::with_capacity(endpoints.len() * 2 - 1);
	    new_endpoints.push(endpoints[0]);
	    for w in endpoints.windows(2) {
		let (a, b) = w[0];
		let (c, d) = w[1];
		let mediant = (a + c, b + d);
		let val = mediant.0 as f32 / mediant.1 as f32;
		if val > 1.0 && val <= self.framing_interval + 1e-6 {
		    insertion_order.push(mediant);
		    new_endpoints.push(mediant);
		}
		new_endpoints.push(w[1]);
		if insertion_order.len() >= num_notes {
		    break;
		}
	    }
	    if new_endpoints.len() == endpoints.len() {
		// No new mediants fit; bail to avoid infinite loop.
		break;
	    }
	    endpoints = new_endpoints;
	}

	// Place 1/1 first so the lowest scale step is the base freq
	// itself; then the inserted mediants in their complexity
	// order. Truncate to num_notes.
	let mut out: Vec<f32> = Vec::with_capacity(num_notes);
	out.push(self.base_freq);
	for (n, d) in insertion_order.iter().take(num_notes - 1) {
	    out.push(self.base_freq * (*n as f32 / *d as f32));
	}
	while out.len() < num_notes {
	    // Pad by repeating the framing interval upward if we ran
	    // out of mediants. Rare with reasonable num_notes; keeps
	    // the scale length contract.
	    let last = out[out.len() - 1];
	    out.push(last * self.framing_interval);
	}
	// Sort ascending by pitch. Insertion-order placement above
	// determines *which* ratios live in the scale (the simplest
	// num_notes); sorting them by pitch makes adjacent keyboard
	// keys play adjacent pitches, matching every other tuning
	// system in the synth.
	out.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
	out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f32, b: f32, eps: f32) -> bool {
	(a - b).abs() < eps
    }

    #[test]
    fn first_note_is_base() {
	let s = SternBrocotScale::new(440.0, 2.0);
	let scale = s.generate_scale(1);
	assert_eq!(scale, vec![440.0]);
    }

    #[test]
    fn ascending_monotonic() {
	let s = SternBrocotScale::new(220.0, 2.0);
	let scale = s.generate_scale(40);
	assert_eq!(scale.len(), 40);
	for w in scale.windows(2) {
	    assert!(w[1] >= w[0], "non-monotonic at {:?}", w);
	}
    }

    #[test]
    fn early_mediants_present_in_output() {
	let s = SternBrocotScale::new(1.0, 2.0);
	let scale = s.generate_scale(8);
	// The simplest 7 mediants of (1/1, 2/1) by complexity are:
	// 3/2 (pass 1); 4/3, 5/3 (pass 2); 5/4, 7/5, 8/5, 7/4 (pass 3).
	// All of these plus 1/1 should appear in the 8-note output,
	// regardless of pitch ordering within the output.
	let expected = [1.0, 5.0/4.0, 4.0/3.0, 7.0/5.0, 3.0/2.0, 8.0/5.0, 5.0/3.0, 7.0/4.0];
	for exp in expected {
	    assert!(
		scale.iter().any(|f| approx(*f, exp, 1e-4)),
		"missing {exp} in {:?}", scale,
	    );
	}
    }

    #[test]
    fn produces_requested_count() {
	let s = SternBrocotScale::new(220.0, 2.0);
	let scale = s.generate_scale(40);
	assert_eq!(scale.len(), 40);
	for f in &scale {
	    assert!(*f > 0.0 && f.is_finite());
	}
    }
}
