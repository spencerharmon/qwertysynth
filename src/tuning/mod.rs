pub mod harmonic;
pub mod mos;
pub mod lattice;
pub mod stern_brocot;
pub mod scala;

/// Tuning systems generate a fixed-length vector of frequencies (Hz)
/// to be assigned to scale degrees, lowest first.
pub trait TuningSystem {
    fn generate_scale(&self, num_notes: usize) -> Vec<f32>;
}
