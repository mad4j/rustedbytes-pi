

use rustedbytes_pi::compute_pi;



fn main() {

    let digits = 1_000_000_u64;

    howlast::howlast!(ellapsed_time, result => { compute_pi(digits) });

    println!("π ≈ {}", result);
    println!("Elapsed time: {:?}", ellapsed_time);
}
