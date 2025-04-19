

use rustedbytes_pi::compute_pi;
use clap::Parser;


fn _get_pi_digits() -> String {
    include_str!("../tests/pi_digits.txt").to_string()
}

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
}

fn main() {
    let args = Cli::parse();

    howlast::howlast!(compute_time, pi => { compute_pi(args.digits) });

    println!("{}", pi);
    println!("Compute Time: {:?}", compute_time);

}
