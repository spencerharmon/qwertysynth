pub struct Scale {
    frequencies: Vec<f32>,
}

impl Scale {
    pub fn new(frequencies: Vec<f32>) -> Scale {
	Scale { frequencies: frequencies }
    }

    pub fn get_frequencies_vector(self) -> Vec<f32> {
	return self.frequencies;
    }
}
