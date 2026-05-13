use crate::tuning::TuningSystem;
use std::fmt;

/// Parsed contents of a Scala (.scl) scale file. The ratios vector
/// contains N entries above the implicit 1/1 unison; the last entry
/// is the period (typically 2/1 = octave, sometimes 3/1 = tritave or
/// other). One period therefore contains N degrees indexed 0..N where
/// degree 0 is unison and degree N would be the unison of the next
/// period.
#[derive(Clone, Debug)]
pub struct ScalaFile {
    pub description: String,
    pub ratios: Vec<f32>,
}

#[derive(Debug)]
pub enum ScalaParseError {
    NoDescription,
    NoCount,
    BadCount(String),
    NotEnoughEntries { expected: usize, got: usize },
    BadRatio(String),
    NonPositiveRatio(f32),
}

impl fmt::Display for ScalaParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	match self {
	    Self::NoDescription => write!(f, "missing description line"),
	    Self::NoCount => write!(f, "missing count line"),
	    Self::BadCount(s) => write!(f, "bad count line: {s:?}"),
	    Self::NotEnoughEntries { expected, got } =>
		write!(f, "expected {expected} entries, got {got}"),
	    Self::BadRatio(s) => write!(f, "bad ratio line: {s:?}"),
	    Self::NonPositiveRatio(r) => write!(f, "non-positive ratio: {r}"),
	}
    }
}

impl std::error::Error for ScalaParseError {}

pub fn parse(text: &str) -> Result<ScalaFile, ScalaParseError> {
    // A "line" for our purposes is a non-comment, non-empty line. A
    // pitch entry takes whatever is to the left of any whitespace
    // (Scala convention allows trailing comments after whitespace).
    let mut lines = text.lines().filter_map(|l| {
	if l.starts_with('!') {
	    return None;
	}
	let trimmed = l.trim();
	if trimmed.is_empty() { None } else { Some(trimmed) }
    });

    let description = lines.next().ok_or(ScalaParseError::NoDescription)?.to_string();
    let count_line = lines.next().ok_or(ScalaParseError::NoCount)?;
    let count: usize = count_line
	.split_whitespace()
	.next()
	.ok_or_else(|| ScalaParseError::BadCount(count_line.to_string()))?
	.parse()
	.map_err(|_| ScalaParseError::BadCount(count_line.to_string()))?;

    let mut ratios = Vec::with_capacity(count);
    for line in lines {
	if ratios.len() >= count {
	    break;
	}
	// Take only the first whitespace-separated token; everything
	// after may be a Scala-style trailing comment.
	let token = line.split_whitespace().next().unwrap_or("");
	let r = parse_pitch(token).ok_or_else(|| ScalaParseError::BadRatio(line.to_string()))?;
	if r <= 0.0 || !r.is_finite() {
	    return Err(ScalaParseError::NonPositiveRatio(r));
	}
	ratios.push(r);
    }

    if ratios.len() != count {
	return Err(ScalaParseError::NotEnoughEntries {
	    expected: count,
	    got: ratios.len(),
	});
    }

    Ok(ScalaFile { description, ratios })
}

/// Parse a single pitch token. Presence of a '.' indicates cents;
/// otherwise it's a ratio (either "3/2" or just "2"). Returns the
/// linear ratio above unison, or None on parse failure.
fn parse_pitch(s: &str) -> Option<f32> {
    if s.is_empty() {
	return None;
    }
    if s.contains('.') {
	let cents: f32 = s.parse().ok()?;
	Some(2f32.powf(cents / 1200.0))
    } else if let Some((num, den)) = s.split_once('/') {
	let n: f32 = num.parse().ok()?;
	let d: f32 = den.parse().ok()?;
	if d == 0.0 { None } else { Some(n / d) }
    } else {
	let n: f32 = s.parse().ok()?;
	Some(n)
    }
}

/// Tuning system backed by a parsed .scl file. The file's last ratio
/// is the period; the file's other ratios fill the period's
/// non-unison degrees. To fill `num_notes` we cycle through the
/// degrees, multiplying by the period each time we wrap.
pub struct ScalaScale {
    pub base_freq: f32,
    pub file: ScalaFile,
}

impl ScalaScale {
    pub fn new(base_freq: f32, file: ScalaFile) -> Self {
	Self { base_freq, file }
    }
}

impl TuningSystem for ScalaScale {
    fn generate_scale(&self, num_notes: usize) -> Vec<f32> {
	if self.file.ratios.is_empty() {
	    return vec![self.base_freq; num_notes];
	}
	let n = self.file.ratios.len();
	let period = self.file.ratios[n - 1];
	// Build one period: [1.0, ratios[0], ratios[1], ..., ratios[n-2]]
	// (degree 0 unison + degrees 1..n-1, omitting the period
	// itself which becomes the unison of the next iteration).
	let mut period_ratios: Vec<f32> = Vec::with_capacity(n);
	period_ratios.push(1.0);
	for r in &self.file.ratios[..n - 1] {
	    period_ratios.push(*r);
	}
	let pn = period_ratios.len();
	(0..num_notes)
	    .map(|i| {
		let octave = (i / pn) as i32;
		let degree = i % pn;
		self.base_freq * period_ratios[degree] * period.powi(octave)
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

    const PYTH_DIATONIC: &str = "\
! pyth.scl
!
Pythagorean diatonic
 7
!
 9/8
 81/64
 4/3
 3/2
 27/16
 243/128
 2/1
";

    #[test]
    fn parses_pythagorean_ratios() {
	let f = parse(PYTH_DIATONIC).expect("parse");
	assert_eq!(f.description, "Pythagorean diatonic");
	assert_eq!(f.ratios.len(), 7);
	assert!(approx(f.ratios[0], 9.0/8.0, 1e-5));
	assert!(approx(f.ratios[6], 2.0, 1e-5));
    }

    #[test]
    fn parses_cents_entries() {
	let text = "test\n2\n100.0\n1200.0\n";
	let f = parse(text).expect("parse");
	assert!(approx(f.ratios[0], 2f32.powf(100.0/1200.0), 1e-5));
	assert!(approx(f.ratios[1], 2.0, 1e-5));
    }

    #[test]
    fn parses_integer_entries_as_ratios() {
	let text = "test\n2\n3\n9\n";
	let f = parse(text).expect("parse");
	assert!(approx(f.ratios[0], 3.0, 1e-5));
	assert!(approx(f.ratios[1], 9.0, 1e-5));
    }

    #[test]
    fn skips_comments_and_blanks() {
	let text = "! header comment\n\nMy scale\n!mid\n2\n!\n3/2\n2/1\n";
	let f = parse(text).expect("parse");
	assert_eq!(f.description, "My scale");
	assert_eq!(f.ratios.len(), 2);
    }

    #[test]
    fn rejects_count_mismatch() {
	let text = "test\n3\n3/2\n2/1\n";
	assert!(matches!(parse(text), Err(ScalaParseError::NotEnoughEntries { .. })));
    }

    #[test]
    fn rejects_negative_ratio() {
	let text = "test\n1\n-3/2\n";
	assert!(matches!(parse(text), Err(ScalaParseError::NonPositiveRatio(_))));
    }

    #[test]
    fn rejects_zero_ratio() {
	let text = "test\n1\n0/1\n";
	assert!(matches!(parse(text), Err(ScalaParseError::NonPositiveRatio(_))));
    }

    #[test]
    fn ignores_trailing_text_after_token() {
	let text = "test\n2\n3/2 G dominant\n2/1 octave\n";
	let f = parse(text).expect("parse");
	assert!(approx(f.ratios[0], 1.5, 1e-5));
	assert!(approx(f.ratios[1], 2.0, 1e-5));
    }

    #[test]
    fn scale_repeats_per_period() {
	let f = parse(PYTH_DIATONIC).expect("parse");
	let s = ScalaScale::new(100.0, f);
	let scale = s.generate_scale(14);
	// 7 ratios in file, last is the period (2/1). One period =
	// 7 degrees (unison + 6 non-period ratios). The 7th and 14th
	// outputs should be exact octaves of the 0th.
	assert!(approx(scale[7], scale[0] * 2.0, 1e-3));
    }

    #[test]
    fn scale_fills_requested_count() {
	let f = parse(PYTH_DIATONIC).expect("parse");
	let s = ScalaScale::new(220.0, f);
	let scale = s.generate_scale(40);
	assert_eq!(scale.len(), 40);
	for w in scale.windows(2) {
	    assert!(w[1] > w[0], "non-monotonic at {:?}", w);
	}
    }

    #[test]
    fn first_note_is_base_freq() {
	let f = parse(PYTH_DIATONIC).expect("parse");
	let s = ScalaScale::new(440.0, f);
	let scale = s.generate_scale(1);
	assert!(approx(scale[0], 440.0, 1e-5));
    }
}
