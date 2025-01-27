use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

enum Direction {
    North,
    East,
    South,
    West,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Direction::North),
            ">" => Ok(Direction::East),
            "v" => Ok(Direction::South),
            "<" => Ok(Direction::West),
            _ => Err("Invalid direction string"),
        }
    }
}

struct Santa {
    position: (isize, isize),
    visited_houses: HashSet<(isize, isize)>,
}

impl Santa {
    fn new() -> Self {
        Santa {
            position: (0, 0),
            visited_houses: HashSet::from([(0, 0)]),
        }
    }

    fn move_sleigh(&mut self, direction: &Direction) {
        match direction {
            Direction::North => self.position = (self.position.0, self.position.1 + 1),
            Direction::East => self.position = (self.position.0 + 1, self.position.1),
            Direction::South => self.position = (self.position.0, self.position.1 - 1),
            Direction::West => self.position = (self.position.0 - 1, self.position.1),
        };

        self.visited_houses.insert(self.position);
    }
}

fn parse_input<T: AsRef<Path>>(file_path: T) -> io::Result<Vec<Direction>> {
    // Open input file
    let input = fs::read_to_string(file_path)?;

    Ok(input
        .chars()
        .filter_map(|s| String::from(s).parse::<Direction>().ok())
        .collect())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let directions = parse_input("days/day03/input")?;
    let parse_time = t0.elapsed();

    // Compute solution and time it
    let t1 = Instant::now();

    // Number of houses visited by one Santa
    let mut santa1 = Santa::new();
    for direction in &directions {
        santa1.move_sleigh(direction);
    }
    let num_visited_houses_1 = santa1.visited_houses.len();

    // Number of houses visited by Santa and robo-Santa
    let mut santa2 = Santa::new();
    let mut robo_santa = Santa::new();
    for direction in directions.iter().step_by(2) {
        santa2.move_sleigh(direction);
    }
    for direction in directions.iter().skip(1).step_by(2) {
        robo_santa.move_sleigh(direction);
    }
    let num_visited_houses_2 = santa2
        .visited_houses
        .union(&robo_santa.visited_houses)
        .count();

    let solution_time = t1.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let solution_time =
        solution_time.as_millis() as f64 + (solution_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Solution:\nTook {:.6}ms\nNumber of houses visited by one Santa: {}\nNumber of houses visited by Santa and Robo-Santa: {}\n",
        solution_time, num_visited_houses_1, num_visited_houses_2
    );

    Ok(())
}
