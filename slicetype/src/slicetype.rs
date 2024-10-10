use std::io;
use std::thread;
use std::time::Duration;

pub fn slicetype() {
    let mut numbers: Vec<i32> = Vec::new();
    let mut input = String::new();

    println!("Enter numbers (type 'done' to finish):");

    loop {
        // Read user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed_input = input.trim();

        // Check if the user wants to stop entering numbers
        if trimmed_input.eq_ignore_ascii_case("done") {
            break;
        }

        // Try to parse the input as an integer
        match trimmed_input.parse::<i32>() {
            Ok(num) => numbers.push(num), // Store the number in the vector
            Err(_) => println!("Please enter a valid number or 'done' to finish."),
        }

        // Clear the input for the next iteration
        input.clear();
    }

    println!("You entered: {:?}", numbers);

    // Wait for 50 seconds
    println!("Waiting for 50 seconds...");
    thread::sleep(Duration::new(50, 0));

    // Print the stored numbers after the delay
    println!("Stored numbers after 50 seconds: {:?}", numbers);
}

/*
Explanation

    Dynamic Array Creation:
        We use Vec<i32> to create a dynamic array that can grow as needed. This allows us to store an arbitrary number of integers.
    User Input Loop:
        The program prompts the user to enter numbers. It continues to read input until the user types "done".
        If the input can be parsed into an i32, it gets pushed into the numbers vector. If parsing fails, it prompts the user again.
    Delay:
        After collecting inputs, the program waits for 50 seconds using thread::sleep(Duration::new(50, 0)).
    Output:
        Finally, it prints the stored numbers after the delay.

*/
//simple slice type example
// pub fn slicetype() {
//     //An array representing soundwave amplitudes at different time
//     //intevals
//     let wave: [f64; 10] = [0.0, 0.5, 1.0, 0.5, 0.0, -0.5, -1.0, -0.5, 0.0, 0.5];
//     //create a slice to analyze the first half of the wave
//     let first_half: &[f64] = &wave[0..5];
//     //printing the first hald of the waveform
//     println!("First half of the waveform: {:?}", first_half);
// }
