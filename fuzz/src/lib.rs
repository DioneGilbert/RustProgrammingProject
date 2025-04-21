pub fn parse_complex(data: &[u8]) -> bool{
	if data.len() == 5 {
		if data[0] == b'F' && data[1] == b'U' && data[2] == b'Z' && data[3] == b'Z' && data[4] == b'I' && data[5] == b'T' {
			return true
		}
	}
    return true;
}
