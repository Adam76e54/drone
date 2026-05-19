use crate::control::filtering::lowpass_alpha;

/**
 * Things to add: 
 * 1. FF based on setpoint derivative (with filtering and averaging)
 * 2. D-term on measurement derivative instead of error derivative
 * 3. Dynamic kd (small for regular flying, high for fast flying in dirty air)
 * 4. I-term relaxing, rotating, and windup handling (supressing when the setpoint change is rapid, stuff like that)
 * 5. Antigravity (increase I-term when throttling is high) to avoid noise dipping
 * 6. Throttle PID Attenuation (TPA)
 * 7. Rate curves (RC, expo, super rate)
 */

#[derive(Clone, Copy)]
pub struct PIDF {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub kff: f32, // Feed-forward gain
}
#[derive(Clone, Copy)]
pub struct Controller {
    coefficients: PIDF,
    integral_limit: f32,
    filter_hz: f32, // for filtering the derivative term
    
    integral: f32,
    previous_error: Option<f32>,
    filtered_derivative: Option<f32>,
}

impl Controller {
    pub const fn new(coefficients: PIDF, integral_limit: f32, filter_hz: f32) -> Self {
        Self {
            coefficients, 
            integral_limit,
            filter_hz,
            integral: 0.0,
            previous_error: None,
            filtered_derivative: None,
        }
    }

    // Returns the adjustment. Note that we'll keep dt external because we'll have a few of these controllers 
    // Going at the same time so we don't want them all internally reading Instant::now() 
    pub fn update(&mut self, setpoint: f32, measurement: f32, dt_s: f32) -> f32 {
        if dt_s <= 0.0 { return 0.0; }

        let error = setpoint - measurement;

        // Compute integral
        self.integral = self.integral + error * dt_s;
        if self.integral_limit > 0.0 {
            self.integral = self.integral.clamp(-self.integral_limit, self.integral_limit);
        }

        // Compute filtered deriviative
        let raw_derivative = match self.previous_error {
            Some(prev) => (error - prev) / dt_s,
            None => 0.0,
        };
        self.previous_error = Some(error);

        let derivative = match self.filtered_derivative {
            None => {
                self.filtered_derivative = Some(raw_derivative);
                raw_derivative
            }

            Some(prev_filtered) => {
                let alpha = lowpass_alpha(self.filter_hz, dt_s);
                let filtered = prev_filtered + alpha * (raw_derivative - prev_filtered);
                self.filtered_derivative = Some(filtered);
                filtered
            }
        };


        // Compute adjustments
        let p = error * self.coefficients.kp; 
        let i = self.integral * self.coefficients.ki;
        let d = derivative * self.coefficients.kd;
        let f = setpoint * self.coefficients.kff;

        p + i + d + f
    }


}