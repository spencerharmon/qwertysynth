use crate::tuning::TuningSystem;

/// MOS (Moment of Symmetry) / generator-stack tuning system.
/// Stack `generator` `mos_size` times, reduce each into the framing
/// interval, sort. That gives one period; repeat upward to fill the
/// requested number of notes.
pub struct MosScale {
    pub base_freq: f32,
    pub generator: f32,
    pub framing_interval: f32,
    pub mos_size: u8,
}

impl MosScale {
    pub fn new(base_freq: f32, generator: f32, framing_interval: f32, mos_size: u8) -> Self {
	// Defensive clamps so degenerate slider values never crash the
	// generator. generator must be in (1, framing_interval); a
	// generator equal to or above the framing interval reduces in
	// one step and produces a unison scale.
	let framing_interval = framing_interval.max(1.001);
	let generator = generator.clamp(1.001, framing_interval - 0.001);
	let mos_size = mos_size.max(1);
	Self { base_freq, generator, framing_interval, mos_size }
    }

    /// Build one period: stack the generator `mos_size` times, reduce
    /// each into [1, framing_interval), sort ascending.
    fn period_ratios(&self) -> Vec<f32> {
	let mut ratios: Vec<f32> = (0..self.mos_size)
	    .map(|i| {
		let mut r = self.generator.powi(i as i32);
		while r >= self.framing_interval {
		    r /= self.framing_interval;
		}
		while r < 1.0 {
		    r *= self.framing_interval;
		}
		r
	    })
	    .collect();
	ratios.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
	ratios
    }
}

impl TuningSystem for MosScale {
    fn generate_scale(&self, num_notes: usize) -> Vec<f32> {
	let period = self.period_ratios();
	let n = period.len();
	(0..num_notes)
	    .map(|i| {
		let octave = (i / n) as i32;
		let degree = i % n;
		self.base_freq * period[degree] * self.framing_interval.powi(octave)
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

    #[test]
    fn pythagorean_seven_stacks_upward() {
	let mos = MosScale::new(440.0, 1.5, 2.0, 7);
	let scale = mos.generate_scale(7);
	// Stack (3/2) seven times upward, reduce into [1,2), sort:
	// (3/2)^0..6 reduced -> 1, 9/8, 81/64, 3/2, 27/16, 243/128, 729/512
	// sorted: 1, 9/8, 81/64, 729/512, 3/2, 27/16, 243/128
	let expected = [
	    1.0,
	    9.0/8.0,
	    81.0/64.0,
	    729.0/512.0,
	    3.0/2.0,
	    27.0/16.0,
	    243.0/128.0,
	];
	for (got, exp) in scale.iter().zip(expected.iter()) {
	    assert!(approx(got / 440.0, *exp, 1e-4),
		    "got {} vs exp {} * 440", got, exp * 440.0);
	}
    }

    #[test]
    fn ascending_monotonic() {
	let mos = MosScale::new(220.0, 1.5, 2.0, 7);
	let scale = mos.generate_scale(40);
	assert_eq!(scale.len(), 40);
	for w in scale.windows(2) {
	    assert!(w[1] > w[0], "non-monotonic at {:?}", w);
	}
    }

    #[test]
    fn period_repeats_correctly() {
	let mos = MosScale::new(100.0, 1.5, 2.0, 7);
	let scale = mos.generate_scale(14);
	for i in 0..7 {
	    assert!(approx(scale[i + 7], scale[i] * 2.0, 1e-3),
		    "period 1 should be 2x period 0 at index {}", i);
	}
    }

    #[test]
    fn degenerate_generator_clamped() {
	let mos = MosScale::new(100.0, 0.5, 2.0, 7);
	let scale = mos.generate_scale(7);
	assert_eq!(scale.len(), 7);
	for w in scale.windows(2) {
	    assert!(w[1] >= w[0]);
	}
    }
}
