use std::f64::consts::PI;

// Function to find the Greatest Common Divisor (GCD) using Euclidean algorithm
fn gcd(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Structure to represent sinusoidal components
struct SinusoidalComponent {
    period: f64,
}

fn is_periodic(components: &[SinusoidalComponent]) -> Option<f64> {
    if components.is_empty() {
        return None;
    }

    let mut hcd_periods = components[0].period;
    for component in &components[1..] {
        hcd_periods = gcd(hcd_periods, component.period);
    }

    if hcd_periods.is_infinite() {
        None
    } else {
        Some(hcd_periods)
    }
}

fn main() {
    // Define the sinusoidal components for each problem
    let problem_a = vec![
        SinusoidalComponent { period: 2.0 * PI / PI },      // 2sin(pi*t)
        SinusoidalComponent { period: 2.0 * PI / 1.0 },     // cos(t)
    ];

    let problem_b = vec![
        SinusoidalComponent { period: 2.0 * PI / (1.5 * PI) }, // cos^2(3*pi*n)
    ];

    if let Some(lowest_period) = is_periodic(&problem_a) {
        println!("Problem a) is periodic with the lowest fundamental period: {}", lowest_period);
    } else {
        println!("Problem a) is aperiodic.");
    }

    if let Some(lowest_period) = is_periodic(&problem_b) {
        println!("Problem b) is periodic with the lowest fundamental period: {}", lowest_period);
    } else {
        println!("Problem b) is aperiodic.");
    }
}
