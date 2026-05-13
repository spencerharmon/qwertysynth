use crate::tuning::TuningSystem;

pub struct HarmonicSeries {
    pub fundamental: f32,
    pub start_harmonic: u32,
}

impl HarmonicSeries {
    pub fn new(fundamental: f32, start_harmonic: u32) -> Self {
	Self { fundamental, start_harmonic: start_harmonic.max(1) }
    }
}

impl TuningSystem for HarmonicSeries {
    fn generate_scale(&self, num_notes: usize) -> Vec<f32> {
	(0..num_notes)
	    .map(|i| self.fundamental * (self.start_harmonic + i as u32) as f32)
	    .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_at_correct_harmonic() {
	let hs = HarmonicSeries::new(100.0, 8);
	let scale = hs.generate_scale(4);
	assert_eq!(scale, vec![800.0, 900.0, 1000.0, 1100.0]);
    }

    #[test]
    fn start_clamped_to_one() {
	let hs = HarmonicSeries::new(100.0, 0);
	let scale = hs.generate_scale(2);
	assert_eq!(scale, vec![100.0, 200.0]);
    }

    #[test]
    fn ascending_monotonic() {
	let hs = HarmonicSeries::new(110.0, 8);
	let scale = hs.generate_scale(40);
	assert_eq!(scale.len(), 40);
	for w in scale.windows(2) {
	    assert!(w[1] > w[0]);
	}
    }
}
