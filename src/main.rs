use indicatif::{ProgressBar, ProgressStyle};
use num_bigint::BigInt;

fn calc_pi() -> Vec<u8> {
    // This function calculates pi using the Bailey-Borwein-Plouffe formula\
    let mut q = BigInt::from(1);
    let mut r = BigInt::from(0);
    let mut t = BigInt::from(1);
    let mut k = BigInt::from(1);
    let mut n = BigInt::from(3);
    let mut l = BigInt::from(3);
    let mut first = true;
    let mut count = 0;
    let max_digits = 1000; // Replace with the desired number of digits

    let mut digits = Vec::with_capacity(max_digits);

    let bar = ProgressBar::new(max_digits as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {percent:>3}% [ETA: {eta}] {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    while count < max_digits {
        if &q * 4 + &r - &t < &n * &t {
            //print!("{}", n);
            if first {
                //print!(".");
                first = false;
            }
            if !first {
                digits.push((n.to_string().chars().next().unwrap() as u8) - b'0');
            }
            let nr = (&r - &n * &t) * 10;
            n = (&q * 3 + &r) * 10 / &t - &n * 10;
            q *= 10;
            r = nr;
            count += 1;
            bar.inc(1);
        } else {
            let nr = (&q * 2 + &r) * &l;
            let nn = (&q * &k * 7 + 2 + &r * &l) / (&t * &l);
            q *= &k;
            t *= &l;
            l += 2;
            k += 1;
            n = nn;
            r = nr;
        }
    }
    bar.finish();

    println!("Pi calculated to {} digits", max_digits);
    println!("Pi: 3.");
    for i in 0..digits.len() {
        if i % 50 == 0 && i != 0 {
            println!();
        }
        print!("{}", digits[i]);
    }
    println!();

    digits

}


fn main() {
    calc_pi();
}
