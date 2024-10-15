use rand::Rng; // Ensure to add `rand` to your Cargo.toml

#[derive(Debug)]
enum QuantumState {
    GroundState,
    ExcitedState(f64), // Energy level associated with the excited state
    Superposition {
        amplitude_a: f64, // Amplitude for state A
        amplitude_b: f64, // Amplitude for state B
    },
    Measured(String), // Result of measurement (e.g., "A" or "B")
}

fn measure(state: QuantumState) -> QuantumState {
    match state {
        QuantumState::GroundState => {
            println!("Measuring ground state.");
            QuantumState::Measured("Ground".to_string())
        }
        QuantumState::ExcitedState(energy) => {
            println!("Measuring excited state with energy level: {}", energy);
            QuantumState::Measured("Excited".to_string())
        }
        QuantumState::Superposition {
            amplitude_a,
            amplitude_b,
        } => {
            let mut rng = rand::thread_rng();
            let result = if rng.gen::<f64>() < amplitude_a {
                "A"
            } else {
                "B"
            };
            println!(
                "Measuring superposition: A({}) vs B({}) -> Result: {}",
                amplitude_a, amplitude_b, result
            );
            QuantumState::Measured(result.to_string())
        }
        QuantumState::Measured(ref result) => {
            // Borrowing the result
            println!("Already measured: {}", result);
            state // Return the measured state without change
        }
    }
}

fn main() {
    let ground_state = QuantumState::GroundState;
    let excited_state = QuantumState::ExcitedState(5.0);
    let superposition_state = QuantumState::Superposition {
        amplitude_a: 0.7,
        amplitude_b: 0.3,
    };

    let states = vec![ground_state, excited_state, superposition_state];

    for state in states {
        let measured_state = measure(state);
        println!("Final state after measurement: {:?}", measured_state);
    }
}
