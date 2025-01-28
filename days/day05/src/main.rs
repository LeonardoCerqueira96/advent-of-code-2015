use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

use itertools::Itertools;

fn is_vowel(c: &char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    vowels.iter().any(|v| v == c)
}

fn is_nice_string_1<T: AsRef<str>>(string: &T) -> bool {
    // Nice strings countain at least 3 vowels
    let vowel_count = string.as_ref().chars().filter(is_vowel).count();
    if vowel_count < 3 {
        return false;
    }

    // Nice strings have at least one occurrence of the same letter twice in a row
    let has_repeated_letter = string
        .as_ref()
        .chars()
        .tuple_windows()
        .any(|(c1, c2)| c1 == c2);
    if !has_repeated_letter {
        return false;
    }

    // Nice strings can't have any of the substrings below
    let naughty_substrs = ["ab", "cd", "pq", "xy"];
    let has_naughty_substr = naughty_substrs
        .iter()
        .any(|&substr| string.as_ref().contains(substr));
    if has_naughty_substr {
        return false;
    }

    true
}

fn is_nice_string_2<T: AsRef<str>>(string: &T) -> bool {
    // Nice strings have at least one pair of characters that repeat
    let mut has_repeating_pairs = false;
    let mut pairs_map = HashMap::new();
    string
        .as_ref()
        .chars()
        .tuple_windows()
        .enumerate()
        .for_each(|(i, (c1, c2))| {
            if let Some(pos) = pairs_map.get(&(c1, c2)) {
                if i > pos + 1 {
                    has_repeating_pairs = true;
                }
                return;
            }

            pairs_map.insert((c1, c2), i);
        });
    if !has_repeating_pairs {
        return false;
    }

    // Nice strings have at least one letter which repeats with exactly one letter between them
    let has_repeated_letter = string
        .as_ref()
        .chars()
        .tuple_windows()
        .any(|(c1, _c2, c3)| c1 == c3);
    if !has_repeated_letter {
        return false;
    }

    true
}

fn parse_input<T: AsRef<Path>>(file_path: T) -> io::Result<Vec<String>> {
    // Open input file
    let input = File::open(file_path)?;
    let input_buf = BufReader::new(input);

    let mut strings = Vec::new();
    for line in input_buf.lines() {
        let line = line?;

        strings.push(line);
    }

    Ok(strings)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let strings = parse_input("days/day05/input")?;
    let parse_time = t0.elapsed();

    // Compute solution and time it
    let t1 = Instant::now();

    let nice_strings_count_1 = strings.iter().filter(is_nice_string_1).count();
    let nice_strings_count_2 = strings.iter().filter(is_nice_string_2).count();

    let solution_time = t1.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let solution_time =
        solution_time.as_millis() as f64 + (solution_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Solution:\nTook {:.6}ms\nNumber of nice strings using old rules: {}\nNumber of nice strings using new rules: {}\n",
        solution_time, nice_strings_count_1, nice_strings_count_2
    );

    Ok(())
}
