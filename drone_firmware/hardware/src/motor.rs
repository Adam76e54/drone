
pub struct Frame {
    // A 16-bit word SSSSSSSSSSSTCCCC where S is an 11-bit payload, T is a a telemtry bit and CCCC is a 4-bit checksum
    pub word: u16,
}

impl Frame {
    pub fn new(value: u16, telemetry: bool) -> Self {
        // clamp to 11 bits
        let value = core::cmp::min(value, 0x7FF);

        let telemetry_bit = if telemetry { 1u16 } else { 0u16 };

        // payload is 12 bit, the combination of the value and telemetry bit. 
        let payload = (value << 1) | telemetry_bit;

        // DShot checksum is the XOR of the three nibbles of the payload
        let checksum = {
            // shift the payload down, then mask it with 0xF to get each nibble. 
            let b0 = ((payload >> 0) & 0xF) as u8;
            let b1 = ((payload >> 4) & 0xF) as u8;
            let b2 = ((payload >> 8) & 0xF) as u8;
            // Now XOR them
            b0 ^ b1 ^ b2 & 0xF
        } as u16;

        let word = (payload << 4) | checksum;

        Self { word }
    }

    pub fn to_ticks(
        &self, 
        buf: &mut [u16; 16], // timing buffer
        low_length: u16, 
        high_length: u16,
    ) {
        for (i, bit_index) in (0..16).rev().enumerate() {
            let bit_is_one = (self.word & (1 << bit_index)) != 0;
            buf[i] = if bit_is_one { high_length } else { low_length };
        }
    }
}

