use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug)]
enum Operand {
    Address(String),
    Number(isize),
}

impl FromStr for Operand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(number) = s.parse() {
            Ok(Self::Number(number))
        } else {
            Ok(Self::Address(s.to_string()))
        }
    }
}

#[derive(Debug)]
enum Operation {
    Assign(Operand),
    Not(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    LShift(Operand, Operand),
    RShift(Operand, Operand),
}

fn parse_input<T: AsRef<Path>>(file_path: T) -> io::Result<HashMap<String, Operation>> {
    // Open input file
    let input = File::open(file_path)?;
    let input_buf = BufReader::new(input);

    let mut circuit = HashMap::new();

    for line in input_buf.lines() {
        let line = line?;

        let operands: Vec<&str> = line.split_ascii_whitespace().collect();
        let (target, operation) = match operands[..] {
            [number, "->", target] => (
                target,
                Operation::Assign(Operand::from_str(number).unwrap()),
            ),
            ["NOT", operand, "->", target] => {
                (target, Operation::Not(Operand::from_str(operand).unwrap()))
            }
            [operand1, "AND", operand2, "->", target] => (
                target,
                Operation::And(
                    Operand::from_str(operand1).unwrap(),
                    Operand::from_str(operand2).unwrap(),
                ),
            ),
            [operand1, "OR", operand2, "->", target] => (
                target,
                Operation::Or(
                    Operand::from_str(operand1).unwrap(),
                    Operand::from_str(operand2).unwrap(),
                ),
            ),
            [operand, "LSHIFT", number, "->", target] => (
                target,
                Operation::LShift(
                    Operand::from_str(operand).unwrap(),
                    Operand::from_str(number).unwrap(),
                ),
            ),
            [operand, "RSHIFT", number, "->", target] => (
                target,
                Operation::RShift(
                    Operand::from_str(operand).unwrap(),
                    Operand::from_str(number).unwrap(),
                ),
            ),
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input")),
        };

        circuit.insert(target.to_string(), operation);
    }

    Ok(circuit)
}

fn get_final_wire_value(
    wire: &str,
    circuit: &HashMap<String, Operation>,
    lookup_table: &mut HashMap<String, isize>,
) -> isize {
    if let Some(value) = lookup_table.get(wire) {
        *value
    } else {
        let value = match circuit.get(wire).unwrap() {
            Operation::Assign(operand) => {
                match operand {
                    Operand::Address(wire) => get_final_wire_value(wire, circuit, lookup_table),
                    Operand::Number(number) => *number,
                }
            }
            Operation::Not(operand) => {
                let number = match operand {
                    Operand::Address(wire) => get_final_wire_value(wire, circuit, lookup_table),
                    Operand::Number(number) => *number,
                };

                !number
            }
            Operation::And(operand1, operand2) => {
                let number1 = match operand1 {
                    Operand::Address(wire) => get_final_wire_value(wire, circuit, lookup_table),
                    Operand::Number(number) => *number,
                };

                let number2 = match operand2 {
                    Operand::Address(wire) => get_final_wire_value(wire, circuit, lookup_table),
                    Operand::Number(number) => *number,
                };

                number1 & number2
            }
            Operation::Or(operand1, operand2) => {
                let number1 = match operand1 {
                    Operand::Address(wire) => get_final_wire_value(wire, circuit, lookup_table),
                    Operand::Number(number) => *number,
                };

                let number2 = match operand2 {
                    Operand::Address(wire) => get_final_wire_value(wire, circuit, lookup_table),
                    Operand::Number(number) => *number,
                };

                number1 | number2
            }
            Operation::LShift(operand1, operand2) => {
                let number1 = match operand1 {
                    Operand::Address(wire) => get_final_wire_value(wire, circuit, lookup_table),
                    Operand::Number(number) => *number,
                };

                let number2 = match operand2 {
                    Operand::Address(wire) => get_final_wire_value(wire, circuit, lookup_table),
                    Operand::Number(number) => *number,
                };

                number1 << number2
            }
            Operation::RShift(operand1, operand2) => {
                let number1 = match operand1 {
                    Operand::Address(wire) => get_final_wire_value(wire, circuit, lookup_table),
                    Operand::Number(number) => *number,
                };

                let number2 = match operand2 {
                    Operand::Address(wire) => get_final_wire_value(wire, circuit, lookup_table),
                    Operand::Number(number) => *number,
                };

                number1 >> number2
            }
        };

        lookup_table.insert(wire.to_string(), value);

        value
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let mut circuit = parse_input("days/day07/input")?;
    let parse_time = t0.elapsed();

    // Compute solution and time it
    let t1 = Instant::now();

    // Get value on 'a' wire
    let mut values_table = HashMap::new();
    let wire_a_value_1 = get_final_wire_value("a", &circuit, &mut values_table);

    // Set 'b' wire to the value of a, and re-run
    *circuit.get_mut("b").unwrap() = Operation::Assign(Operand::Number(wire_a_value_1));
    values_table.clear();
    let wire_a_value_2 = get_final_wire_value("a", &circuit, &mut values_table);

    let solution_time = t1.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let solution_time =
        solution_time.as_millis() as f64 + (solution_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Solution:\nTook {:.6}ms\nFirst value on wire 'a': {}\nSecond value on wire 'a': {}\n",
        solution_time, wire_a_value_1, wire_a_value_2
    );

    Ok(())
}
