/// Parse a MIDI Control Change message and return Some(down) iff it is
/// CC 64 (Sustain). Returns None for any other message (including other
/// CC numbers and non-CC messages).
///
/// Standard MIDI sustain semantics: data2 >= 64 = pedal down,
/// data2 < 64 = pedal up. Channel is ignored — any channel triggers
/// sustain. JACK delivers MIDI messages as raw bytes excluding the
/// running-status optimization, so a CC always arrives as exactly
/// three bytes [status, controller, value].
pub fn parse_cc_sustain(bytes: &[u8]) -> Option<bool> {
    if bytes.len() != 3 {
	return None;
    }
    if bytes[0] & 0xF0 != 0xB0 {
	return None;
    }
    if bytes[1] != 64 {
	return None;
    }
    Some(bytes[2] >= 64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cc64_down() {
	assert_eq!(parse_cc_sustain(&[0xB0, 64, 127]), Some(true));
	assert_eq!(parse_cc_sustain(&[0xB0, 64, 64]), Some(true));
    }

    #[test]
    fn cc64_up() {
	assert_eq!(parse_cc_sustain(&[0xB0, 64, 0]), Some(false));
	assert_eq!(parse_cc_sustain(&[0xB0, 64, 63]), Some(false));
    }

    #[test]
    fn any_channel() {
	assert_eq!(parse_cc_sustain(&[0xB5, 64, 127]), Some(true));
	assert_eq!(parse_cc_sustain(&[0xBF, 64, 0]), Some(false));
    }

    #[test]
    fn other_cc_ignored() {
	assert_eq!(parse_cc_sustain(&[0xB0, 1, 127]), None);
	assert_eq!(parse_cc_sustain(&[0xB0, 11, 127]), None);
    }

    #[test]
    fn non_cc_ignored() {
	assert_eq!(parse_cc_sustain(&[0x90, 64, 127]), None);
	assert_eq!(parse_cc_sustain(&[0x80, 64, 0]), None);
    }

    #[test]
    fn wrong_length() {
	assert_eq!(parse_cc_sustain(&[0xB0, 64]), None);
	assert_eq!(parse_cc_sustain(&[0xB0, 64, 0, 0]), None);
	assert_eq!(parse_cc_sustain(&[]), None);
    }
}
