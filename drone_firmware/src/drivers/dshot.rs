#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct DshotCommand(u16);

impl DshotCommand {
    /// Build a 16-bit DShot frame.
    ///
    /// Frame layout:
    /// - bits 15..=5  : 11-bit throttle / command
    /// - bit 4        : telemetry request
    /// - bits 3..=0   : 4-bit checksum
    ///
    /// Notes:
    /// - Values 0..=47 are reserved DShot commands.
    /// - Values 48..=2047 are throttle values.
    pub fn new(throttle: u16, telemetry: bool) -> Self {
        // Clamp to the 11-bit field used by DShot.
        let throttle = throttle & 0x07ff;

        // Build the 12-bit value that the checksum is calculated from:
        // [11 throttle/command bits | 1 telemetry bit]
        let value = (throttle << 1) | u16::from(telemetry);

        // DShot checksum: XOR the three nibbles of that 12-bit value.
        let checksum = (value ^ (value >> 4) ^ (value >> 8)) & 0x0f;

        // Final 16-bit frame:
        // upper 12 bits = value, lower 4 bits = checksum
        let frame = (value << 4) | checksum;

        Self(frame)
    }

    /// Encode the 16 frame bits into timer compare values, MSB first.
    pub fn get_timings(&self, buf: &mut [u16], high_time: u16, low_time: u16) {

        assert!(buf.len() >= 16);

        // NOTE: iter_mut() obviously returns a mutable iterator. enumerate() returns a tuple of (index, iterator)
        // There's some funkiness in the examples I've seen because rust can unwrap references sometimes 
        // (like println!(iterator) works as well as println!(*iterator) apparantly?)
        for (i, slot) in buf.iter_mut().enumerate() {
            let bit_index = 15 - i;
            let bit_is_set = ((self.0 >> bit_index) & 1) != 0;

            *slot = if bit_is_set { high_time } else { low_time };
        }
    }

    /// Expose the raw 16-bit frame 
    pub fn raw(&self) -> u16 {
        self.0
    }
}

