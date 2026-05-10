use maths::consts::{TAU};

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

    filtered_error: f32,
    filtered_derivative: f32,
}

impl PidController {
    pub const fn new(coefficients: PidCoefficients, limiters: PidLimiters) -> Self {
        Self {
            coefficients, 
            limiters,
            integral: 0.0,
            previous_error: 0.0,
            filtered_error: 0.0,
            filtered_derivative: 0.0,
        }
    }

    // Returns the adjustment. Note that we'll keep dt external because we'll have a few of these controllers 
    // Going at the same time so we don't want them all internally reading Instant::now() 
    pub fn update(&mut self, setpoint: f32, measurement: f32, dt_s: f32) -> f32 {
        let error = setpoint - measurement;
        let integral_limit = self.limiters.integral_limit;

        let p = error * self.coefficients.kp; 

        self.integral = self.integral + error * dt_s;
        if integral_limit > 0.0 {
            self.integral = self.integral.clamp(-integral_limit, integral_limit);
        }

        let d = (error / dt_s) * self.coefficients.kd;
        let f = setpoint * self.coefficients.kff;

        p + self.integral + d + f
    }

    fn lowpas_alpha(&self, cutoff_hz: f32, dt_s: f32) -> f32 {
        if cutoff_hz <= 0.0 {
            return 1.0;
        }

        let rc = 1.0 / (TAU * cutoff_hz);
        dt_s / (dt_s + rc)
    }
}