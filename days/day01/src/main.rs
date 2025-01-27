use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::time::Instant;

fn parse_input<T: AsRef<Path>>(file_path: T) -> io::Result<String> {
    // Open input file
    let input = fs::read_to_string(file_path)?;

    Ok(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let input = parse_input("days/day01/input")?;
    let parse_time = t0.elapsed();

    // Compute solution and time it
    let t1 = Instant::now();

    let mut floor_counter = 0;
    let mut first_basement_visit_pos = 0;
    input.chars().enumerate().for_each(|(i, c)| {
        match c {
            '(' => floor_counter += 1,
            ')' => floor_counter -= 1,
            _ => panic!("Invalid character in input"),
        }

        if floor_counter == -1 && first_basement_visit_pos == 0 {
            first_basement_visit_pos = i + 1;
        }
    });

    let solution_time = t1.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let solution_time =
        solution_time.as_millis() as f64 + (solution_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Solution:\nTook {:.6}ms\nFloor counter: {}\nVisited basement for the first time at position: {}\n",
        solution_time, floor_counter, first_basement_visit_pos
    );

    Ok(())
}
