use crate::tuning::TuningSystem;

/// 5-limit lattice tuning system. Generates ratios of the form
/// `3^a * 5^b * 2^c` for integer (a, b, c) where (a, b) live in a
/// bounded window and c is chosen to reduce each ratio into one
/// octave. The set of unique reduced ratios is one period; the period
/// repeats upward (multiplied by 2) to fill the requested note count.
pub struct LatticeScale {
    pub base_freq: f32,
    pub three_limit: u8,
    pub five_limit: u8,
}

impl LatticeScale {
    pub fn new(base_freq: f32, three_limit: u8, five_limit: u8) -> Self {
	Self { base_freq, three_limit, five_limit }
    }

    fn period_ratios(&self) -> Vec<f32> {
	let a_lo = -(self.three_limit as i32);
	let a_hi = self.three_limit as i32;
	let b_lo = -(self.five_limit as i32);
	let b_hi = self.five_limit as i32;

	let mut ratios: Vec<f32> = Vec::new();
	for a in a_lo..=a_hi {
	    for b in b_lo..=b_hi {
		let mut r = 3f32.powi(a) * 5f32.powi(b);
		while r >= 2.0 { r *= 0.5; }
		while r < 1.0 { r *= 2.0; }
		ratios.push(r);
	    }
	}
	ratios.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
	// Dedupe with epsilon. Two ratios within ~0.01 cents are
	// definitely the same lattice point under a different (a,b)
	// pair (e.g. the syntonic comma cycle).
	let eps = 1e-5;
	ratios.dedup_by(|a, b| (*a - *b).abs() / *b < eps);
	ratios
    }
}

impl TuningSystem for LatticeScale {
    fn generate_scale(&self, num_notes: usize) -> Vec<f32> {
	let period = self.period_ratios();
	if period.is_empty() {
	    return vec![self.base_freq; num_notes];
	}
	let n = period.len();
	(0..num_notes)
	    .map(|i| {
		let octave = (i / n) as i32;
		let degree = i % n;
		self.base_freq * period[degree] * 2f32.powi(octave)
	    })
	    .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f32, b: f32, eps: f32) -> bool {
	(a - b).abs() < eps
    }

    fn contains_ratio(ratios: &[f32], target: f32) -> bool {
	ratios.iter().any(|r| approx(*r, target, 1e-3))
    }

    #[test]
    fn unison_at_zero_window() {
	let l = LatticeScale::new(440.0, 0, 0);
	let scale = l.generate_scale(3);
	assert_eq!(scale, vec![440.0, 880.0, 1760.0]);
    }

    #[test]
    fn classic_5_limit_ratios_present() {
	let l = LatticeScale::new(1.0, 2, 1);
	let period = l.period_ratios();
	// 5-limit just-intonation chromatic-ish set; all of these
	// should be present in a (2, 1) window.
	for target in [1.0, 9.0/8.0, 6.0/5.0, 5.0/4.0, 4.0/3.0, 3.0/2.0,
		       8.0/5.0, 5.0/3.0, 9.0/5.0, 15.0/8.0] {
	    assert!(contains_ratio(&period, target),
		    "missing ratio {target} in period {:?}", period);
	}
    }

    #[test]
    fn ascending_monotonic() {
	let l = LatticeScale::new(220.0, 2, 1);
	let scale = l.generate_scale(40);
	assert_eq!(scale.len(), 40);
	for w in scale.windows(2) {
	    assert!(w[1] > w[0], "non-monotonic at {:?}", w);
	}
    }

    #[test]
    fn period_doubles_per_octave() {
	let l = LatticeScale::new(100.0, 2, 1);
	let period = l.period_ratios();
	let n = period.len();
	let scale = l.generate_scale(n * 2);
	for i in 0..n {
	    assert!(approx(scale[i + n], scale[i] * 2.0, 1e-3));
	}
    }
}
