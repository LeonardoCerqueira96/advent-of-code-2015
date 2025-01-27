use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

struct Present {
    length: usize,
    width: usize,
    height: usize,
}

impl Present {
    fn new(length: usize, width: usize, height: usize) -> Self {
        Present {
            length,
            width,
            height,
        }
    }

    fn calculate_wrapping_amount(&self) -> usize {
        let length_x_width = self.length * self.width;
        let mut min_area = length_x_width;

        let length_x_height = self.length * self.height;
        if length_x_height < min_area {
            min_area = length_x_height;
        }

        let width_x_height = self.width * self.height;
        if width_x_height < min_area {
            min_area = width_x_height;
        }

        2 * length_x_width + 2 * length_x_height + 2 * width_x_height + min_area
    }

    fn calculate_ribbon_amount(&self) -> usize {
        let mut dimensions = [self.length, self.width, self.height];
        dimensions.sort();

        let perimeter = dimensions[0] * 2 + dimensions[1] * 2;
        let volume = self.length * self.width * self.height;

        perimeter + volume
    }
}

fn parse_input<T: AsRef<Path>>(file_path: T) -> io::Result<Vec<Present>> {
    // Open input file
    let input = File::open(file_path)?;
    let input_buf = BufReader::new(input);

    let mut presents = Vec::new();
    for line in input_buf.lines() {
        let line = line?;

        let dimensions: Vec<_> = line.split('x').collect();
        let length = dimensions[0]
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let width = dimensions[1]
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let height = dimensions[2]
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        presents.push(Present::new(length, width, height));
    }

    Ok(presents)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let presents = parse_input("days/day02/input")?;
    let parse_time = t0.elapsed();

    // Compute solution and time it
    let t1 = Instant::now();

    let total_wrapping_required: usize = presents
        .iter()
        .map(Present::calculate_wrapping_amount)
        .sum();

    let total_ribbon_required: usize = presents.iter().map(Present::calculate_ribbon_amount).sum();

    let solution_time = t1.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let solution_time =
        solution_time.as_millis() as f64 + (solution_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Solution:\nTook {:.6}ms\nTotal required amount of wrapping paper: {} square feet\nTotal required amount of ribbon: {} feet\n",
        solution_time, total_wrapping_required, total_ribbon_required
    );

    Ok(())
}
