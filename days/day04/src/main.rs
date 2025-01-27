use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let secret_key = "yzbqklnj";

    // Compute solution and time it
    let t0 = Instant::now();

    let mut suffix_number = 0;
    let mut five_zeros_number = None;
    let mut six_zeros_number = None;
    loop {
        let md5_input = format!("{}{}", secret_key, suffix_number);
        let md5_hash = format!("{:x}", md5::compute(md5_input));

        if &md5_hash[..5] == "00000" {
            if five_zeros_number.is_none() {
                five_zeros_number = Some(suffix_number);
            }

            if md5_hash.chars().nth(5).unwrap() == '0' && six_zeros_number.is_none() {
                six_zeros_number = Some(suffix_number);
            }
        }

        if five_zeros_number.is_some() && six_zeros_number.is_some() {
            break;
        }

        suffix_number += 1;
    }
    let solution_time = t0.elapsed();

    // Print results
    let solution_time =
        solution_time.as_millis() as f64 + (solution_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Solution:\nTook {:.6}ms\nLowest five zeros number: {}\nLowest six zeros number: {}\n",
        solution_time,
        five_zeros_number.unwrap(),
        six_zeros_number.unwrap()
    );

    Ok(())
}
