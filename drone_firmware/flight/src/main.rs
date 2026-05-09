use control::pid::{PidCoefficients, PidController, PidLimiters};
fn main() {
    println!("Hello, world!");
    let coefficients = PidCoefficients {kp: 0.0, ki: 0.0, kd: 0.0, kff: 0.0};
    let limiters: PidLimiters = PidLimiters { integral_limit: 0.0, error_filter_frequency_hz: 0.0, derviative_filter_frequency_hz: 0.0 }
    let mut roll_pid = PidController::new(coefficients, limiters);
    
}
