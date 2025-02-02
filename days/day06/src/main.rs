use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

use regex::Regex;

static GRID_SIZE: usize = 1000;

type Coordinate = (usize, usize);
type Rectangle = (Coordinate, Coordinate);

#[derive(Debug)]
enum Instruction {
    TurnOn(Rectangle),
    TurnOff(Rectangle),
    Toggle(Rectangle),
}

enum Ruleset {
    Part1,
    Part2,
}

struct LightGrid {
    ruleset: Ruleset,
    grid: Vec<usize>,
}

impl LightGrid {
    fn new(ruleset: Ruleset) -> Self {
        LightGrid {
            ruleset,
            grid: vec![0; GRID_SIZE * GRID_SIZE],
        }
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        match self.ruleset {
            Ruleset::Part1 => match *instruction {
                Instruction::TurnOn(((x1, y1), (x2, y2))) => {
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            self.grid[x * GRID_SIZE + y] = 1;
                        }
                    }
                }
                Instruction::TurnOff(((x1, y1), (x2, y2))) => {
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            self.grid[x * GRID_SIZE + y] = 0;
                        }
                    }
                }
                Instruction::Toggle(((x1, y1), (x2, y2))) => {
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            self.grid[x * GRID_SIZE + y] += 1;
                            self.grid[x * GRID_SIZE + y] %= 2;
                        }
                    }
                }
            },
            Ruleset::Part2 => match *instruction {
                Instruction::TurnOn(((x1, y1), (x2, y2))) => {
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            self.grid[x * GRID_SIZE + y] += 1;
                        }
                    }
                }
                Instruction::TurnOff(((x1, y1), (x2, y2))) => {
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            self.grid[x * GRID_SIZE + y] =
                                self.grid[x * GRID_SIZE + y].saturating_sub(1);
                        }
                    }
                }
                Instruction::Toggle(((x1, y1), (x2, y2))) => {
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            self.grid[x * GRID_SIZE + y] += 2;
                        }
                    }
                }
            },
        }
    }
}

fn parse_input<T: AsRef<Path>>(file_path: T) -> io::Result<Vec<Instruction>> {
    // Open input file
    let input = File::open(file_path)?;
    let input_buf = BufReader::new(input);

    let mut instructions = Vec::new();
    let regx = Regex::new(
        r"^(?<instruction>toggle|turn on|turn off)\s+(?<x1>\d+),(?<y1>\d+)\s+through\s+(?<x2>\d+),(?<y2>\d+)$" 
    )
    .unwrap();
    for line in input_buf.lines() {
        let line = line?;

        let caps = regx.captures(&line).ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid input format",
        ))?;

        let instruction_str = caps["instruction"].trim();
        let x1: usize = caps["x1"]
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let y1: usize = caps["y1"]
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let x2: usize = caps["x2"]
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let y2: usize = caps["y2"]
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        let instruction = match instruction_str {
            "turn on" => Instruction::TurnOn(((x1, y1), (x2, y2))),
            "turn off" => Instruction::TurnOff(((x1, y1), (x2, y2))),
            "toggle" => Instruction::Toggle(((x1, y1), (x2, y2))),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid instruction",
                ))
            }
        };
        instructions.push(instruction);
    }

    Ok(instructions)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let instructions = parse_input("days/day06/input")?;
    let parse_time = t0.elapsed();

    // Compute solution and time it
    let t1 = Instant::now();

    let mut light_grid_1 = LightGrid::new(Ruleset::Part1);
    let mut light_grid_2 = LightGrid::new(Ruleset::Part2);

    instructions
        .iter()
        .for_each(|i| light_grid_1.run_instruction(i));
    let lit_count = light_grid_1.grid.iter().filter(|&&s| s == 1).count();

    instructions
        .iter()
        .for_each(|i| light_grid_2.run_instruction(i));
    let total_brightness: usize = light_grid_2.grid.iter().sum();

    let solution_time = t1.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let solution_time =
        solution_time.as_millis() as f64 + (solution_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Solution:\nTook {:.6}ms\nNumber of lights lit: {}\nTotal brightness: {}",
        solution_time, lit_count, total_brightness
    );

    Ok(())
}
