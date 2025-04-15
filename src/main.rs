

use rustedbytes_pi::compute_pi;



fn main() {
    let digits = 100; // Cambia qui per più precisione
    let pi = compute_pi(digits);

    println!("π con {} cifre decimali:\n{}", digits, pi);
}
