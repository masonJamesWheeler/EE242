use std::f64::consts::PI;

// Function to find the Greatest Common Divisor (GCD) using Euclidean algorithm
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Structure to represent sinusoidal components
struct SinusoidalComponent {
    frequency: f64,
}

fn is_periodic(components: &[SinusoidalComponent], signal_type: SignalType) -> Option<f64> {
    if components.is_empty() {
        return None;
    }

    let max_freq = components
        .iter()
        .map(|component| component.frequency)
        .fold(0.0, f64::max);

    if max_freq == 0.0 {
        return None;
    }

    let mut i = 1;
    while i as f64 * max_freq < 100_000.0 {
        let mut found = true;
        for component in components {
            let ratio = component.frequency * i as f64 / max_freq;
            if (ratio - ratio.round()).abs() > 1e-10 {
                found = false;
                break;
            }
        }

        if found {
            match signal_type {
                SignalType::Continuous => return Some(2.0 * PI / (i as f64 * max_freq)),
                SignalType::Discrete => return Some(i as f64 / 2.0), // Divide by 2 to account for the square function
            }
        }

        i += 1;
    }

    None
}

enum SignalType {
    Continuous,
    Discrete,
}

fn main() {
    // Define the sinusoidal components for each problem
    let problem_a = vec![
        SinusoidalComponent { frequency: PI },
        SinusoidalComponent { frequency: 1.0 },
    ];

    let problem_b = vec![
        SinusoidalComponent { frequency: 1.5 * PI },
    ];

    if let Some(lowest_period) = is_periodic(&problem_a, SignalType::Continuous) {
        println!("Problem a) is periodic with the lowest fundamental period: {}", lowest_period);
    } else {
        println!("Problem a) is aperiodic.");
    }

    if let Some(lowest_period) = is_periodic(&problem_b, SignalType::Discrete) {
        println!("Problem b) is periodic with the lowest fundamental period: {}", lowest_period);
    } else {
        println!("Problem b) is aperiodic.");
    }
}
