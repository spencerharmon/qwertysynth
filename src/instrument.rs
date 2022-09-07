use crate::wave_table::WaveTable;

pub struct Instrument {
    pub scale_wave_tables: Vec<WaveTable>
}

impl Instrument {
    pub fn new(scale_wave_tables: Vec<WaveTable>) -> Instrument {
	Instrument { scale_wave_tables: scale_wave_tables }
    }
}
