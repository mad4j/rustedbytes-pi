use clap::Parser;
use rustedbytes_pi::compute_pi;

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS")
)]
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

    /// Avoid to print the computed digits
    #[arg(long, default_value_t = false)]
    no_dump: bool,
}

/// Function to get the reference digits of Pi from a file
fn get_pi_digits() -> String {
    include_str!("../tests/pi_digits.txt").to_string()
}

/// Compare the computed digits of Pi with a reference
fn compare_pi(pi: &str) -> bool {
    let mut result = true;
    let reference_pi = get_pi_digits();
    for (i, (computed_char, reference_char)) in pi.chars().zip(reference_pi.chars()).enumerate() {
        if computed_char != reference_char {
            result = false;
            println!(
                "Mismatch at index {}: computed='{}', reference='{}'",
                i, computed_char, reference_char
            );
        }
    }

    result
}

fn main() {
    // Parse command line arguments
    let args = Cli::parse();

    // Compute the digits of Pi using the Chudnovsky algorithm
    howlast::howlast!(compute_time, pi => { compute_pi(args.digits) });

    // Print the computed digits
    if !args.no_dump {
        println!("{}", pi);
    }

    // Print the time taken to compute the digits
    if args.time {
        println!("Compute Time: {:?}", compute_time);
    }

    // Verify the computed digits against a reference
    if args.verify {
        if pi.len() > get_pi_digits().len() {
            println!(
                "Warning: Only the first {} digits will be verified.",
                get_pi_digits().len()-2
            );
        }
        compare_pi(&pi);
    }
}
