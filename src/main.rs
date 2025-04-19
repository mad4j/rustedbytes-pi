

use rustedbytes_pi::compute_pi;
use clap::Parser;


#[derive(Parser)]
#[command(name = "RustedBytes Pi")]
#[command(about = "Computes digits of Pi")]
#[command(long_about = "Computes digits of Pi using the Chudnovsky algorithm")]
#[command(version = "1.0")]
#[command(author = "Daniele Olmisani <daniele.olmisani@gmail.com>")]
struct Cli {
    /// Number of digits to compute
    #[arg(short, long, default_value_t = 1_000_000)]
    digits: usize,

    /// Print the time taken to compute the digits
    #[arg(short, long, default_value_t = false)]
    time: bool,

    /// Verify the computed digits against a reference
    #[arg(short, long, default_value_t = false)]
    verify: bool,
}


fn get_pi_digits() -> String {
    include_str!("../tests/pi_digits.txt").to_string()
}

fn compare_pi(pi: &str) -> bool {
    let mut result = true;
    let reference_pi = get_pi_digits();
    for (i, (computed_char, reference_char)) in pi.chars().zip(reference_pi.chars()).enumerate() {
        if computed_char != reference_char {
            result = false;
            println!("Mismatch at index {}: computed='{}', reference='{}'", i, computed_char, reference_char);
        }
    }

    result
}


fn main() {
    let args = Cli::parse();

    howlast::howlast!(compute_time, pi => { compute_pi(args.digits) });

    //println!("{}", pi);

    if args.time {
        println!("Compute Time: {:?}", compute_time);
    }


    if args.verify {
        compare_pi(&pi);
    }
}
