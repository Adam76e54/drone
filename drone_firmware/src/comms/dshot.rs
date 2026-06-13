/// I've taken https://github.com/sulami as inspiration
/// Using https://betaflight.com/docs/development/API/Dshot as the reference for the protocol 

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Directionality {
    Normal, 
    Bidirectional,
}

// Consider the 11 throttle/command bits as the payload
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadKind {
    Throttle,
    Command,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frame {
    raw: u16, 
    direction: Directionality,
    kind: PayloadKind,
}

impl Frame {
    /// Builds frame from throttle
    /// Expects a value from 0-2000 (handles the "first 48 are actually commands" logic itself)
    pub fn throttle(speed: u16, request_telemetry: bool, direction: Directionality) -> Option<Self> {
        if speed >= 2000 {
            return None;
        }

        // add on the 48 commands and shift up to make room for telemetry and checksum bits
        let shifted_throttle = (speed + 48) << 5;

        let mut frame = Self{
            raw: shifted_throttle,
            direction: direction, 
            kind: PayloadKind::Throttle,
        };
        // Flip the telemetry bit if requested
        if request_telemetry {
            frame.raw |= 0x10;
        } else {
            frame.raw &= !0x10;
        }

        frame.raw |= frame.checksum();

        Some(frame)

    }

    pub fn command(command: Command, request_telemetry: bool, direction: Directionality) -> Self {
        let mut frame = Self {
            raw: (command as u16) << 5,
            direction: direction,
            kind: PayloadKind::Command,
        };

        // Flip the telemetry bit if requested
        if request_telemetry {
            frame.raw |= 0x10;
        } else {
            frame.raw &= !0x10;
        }

        frame.raw |= frame.checksum();
    
        frame

    }

    fn checksum(&self) -> u16 {
        match self.direction {
            Directionality::Normal => {
                (self.raw ^ (self.raw >> 4) ^ (self.raw >> 8)) & 0x0F
            }
            Directionality::Bidirectional => {
                (!(self.raw ^ (self.raw >> 4) ^ (self.raw >> 8))) & 0x0F
            }
        }
    }

    /// Fills buffer with appropriate ccr (clock's capture/compare register) values based on the dshot protocol
    /// to give the clock for dma (direct memory access).
    ///  
    /// Buffer is sized 17 to drive the line low on the last entry 
    pub fn waveform(&self, buf: &mut [u16; 17], max_ccr: u32) {
        
        let mut bits = self.raw;

        let zero = (max_ccr * 375 / 1000) as u16;
        let one = (max_ccr * 750 / 1000) as u16;

        for value in buf[..16].iter_mut() {
            let single_bit = bits & 0x8000; // 0b1000_0000_0000_0000

            if single_bit == 0 {
                *value = zero;
            } else {
                *value = one;
            }
            bits <<= 1;
        }

        // Drive the line low a the end
        buf[16] = 0; 
    }

}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Command {
    MotorStop = 0,
    /// Wait at least 260ms before next command.
    Beep1 = 1,
    /// Wait at least 260ms before next command.
    Beep2 = 2,
    /// Wait at least 260ms before next command.
    Beep3 = 3,
    /// Wait at least 260ms before next command.
    Beep4 = 4,
    /// Wait at least 260ms before next command.
    Beep5 = 5,
    /// Wait at least 12ms before next command.
    ESCInfo = 6,
    /// Needs 6 transmissions.
    SpinDirection1 = 7,
    /// Needs 6 transmissions.
    SpinDirection2 = 8,
    /// Needs 6 transmissions.
    ThreeDModeOn = 9,
    /// Needs 6 transmissions.
    ThreeDModeOff = 10,
    SettingsRequest = 11,
    /// Needs 6 transmissions. Wait at least 35ms before next command.
    SettingsSave = 12,
    /// Needs 6 transmissions.
    ExtendedTelemetryEnable = 13,
    /// Needs 6 transmissions.
    ExtendedTelemetryDisable = 14,

    // 15-19 are unassigned.
    /// Needs 6 transmissions.
    SpinDirectionNormal = 20,
    /// Needs 6 transmissions.
    SpinDirectonReversed = 21,
    Led0On = 22,
    Led1On = 23,
    Led2On = 24,
    Led3On = 25,
    Led0Off = 26,
    Led1Off = 27,
    Led2Off = 28,
    Led3Off = 29,
    AudioStreamModeToggle = 30,
    SilentModeToggle = 31,
    /// Needs 6 transmissions. Enables individual signal line commands.
    SignalLineTelemetryEnable = 32,
    /// Needs 6 transmissions. Disables individual signal line commands.
    SignalLineTelemetryDisable = 33,
    /// Needs 6 transmissions. Enables individual signal line commands.
    SignalLineContinuousERPMTelemetry = 34,
    /// Needs 6 transmissions. Enables individual signal line commands.
    SignalLineContinuousERPMPeriodTelemetry = 35,

    // 36-41 are unassigned.
    /// 1ºC per LSB.
    SignalLineTemperatureTelemetry = 42,
    /// 10mV per LSB, 40.95V max.
    SignalLineVoltageTelemetry = 43,
    /// 100mA per LSB, 409.5A max.
    SignalLineCurrentTelemetry = 44,
    /// 10mAh per LSB, 40.95Ah max.
    SignalLineConsumptionTelemetry = 45,
    /// 100erpm per LSB, 409500erpm max.
    SignalLineERPMTelemetry = 46,
    /// 16us per LSB, 65520us max.
    SignalLineERPMPeriodTelemetry = 47,
}