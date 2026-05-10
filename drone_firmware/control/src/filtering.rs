use maths::consts::TAU;

// Returns alpha that solves RCy' = x - y when solving via Euler Method 
// (eg. y = voltage out, x = voltage in, for an RC circuit)
// That Euler solution will give: y_i+1 = y_i + dt * (1/RC * (x - y)), then let alpha = dt/RC
// It's known that alpha dt/(dt + rc) is actually a better approximation so we're using that instead
pub fn lowpass_alpha(cutoff_hz: f32, dt_s: f32) -> f32 {
    if cutoff_hz <= 0.0 {
        return 1.0;
    }

    let rc = 1.0 / (TAU * cutoff_hz);
    dt_s / (dt_s + rc)
}