#[derive(Clone, Copy)]

pub struct PidCoefficients {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub kff: f32, // Feed-forward gain
}

pub struct PidLimiters {
    pub integral_limit: f32,
    pub error_filter_frequency_hz: f32,
    pub derviative_filter_frequency_hz: f32,
}
pub struct PidController {
    coefficients: PidCoefficients,
    limiters: PidLimiters,
    
    integral: f32,
    previous_error: f32,
    previous_target: f32,
}

impl PidController {
    pub const fn new(coefficients: PidCoefficients, limiters: PidLimiters) -> Self {
        Self {
            coefficients, 
            limiters,
            integral: 0.0,
            previous_error: 0.0,
            previous_target: 0.0,
        }
    }

    // Returns the adjustment. Note that we'll keep dt external because we'll have a few of these controllers 
    // Going at the same time so we don't want them all internally reading Instant::now() 
    pub fn update(&mut self, target: f32, measurement: f32, dt_s: f32) -> f32 {
        todo!();
    }
}