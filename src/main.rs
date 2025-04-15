use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::str::FromStr;
use num_traits::Zero;

fn chudnovsky_iterations_for_digits(n: usize) -> usize {
    let digits_per_term = 14.181647462725477_f64;
    (n as f64 / digits_per_term).ceil() as usize
}


/// Calcola la radice quadrata con Newton-Raphson ad alta precisione
pub fn sqrt_bigdecimal(num: &BigDecimal, scale: u64) -> BigDecimal {
    if num.is_zero() {
        return BigDecimal::zero();
    }

    let two = BigDecimal::from(2u32);
    let mut x = num.clone();
    let precision = BigDecimal::from_str(&format!("1e-{}", scale)).unwrap();

    // Itera fino a raggiungere la precisione
    loop {
        let next = (&x + num / &x) / &two;
        if (&next - &x).abs() < precision {
            return next.with_scale(scale as i64);
        }
        x = next;
    }
}

fn binary_split(a: i64, b: i64) -> (BigInt, BigInt, BigInt) {
    if b == a + 1 {
        let a_big = BigInt::from(a);
        let six_a = 6 * a;
        let two_a = 2 * a;
        let pab = -((six_a - 5) * (two_a - 1) * (six_a - 1));
        let qab = BigInt::from(10939058860032000i64) * &a_big * &a_big * &a_big;
        let rab = BigInt::from(pab) * (BigInt::from(545140134i64) * &a_big + BigInt::from(13591409i64));
        (BigInt::from(pab), qab, rab)
    } else {
        let m = (a + b) / 2;
        let (pam, qam, ram) = binary_split(a, m);
        let (pmb, qmb, rmb) = binary_split(m, b);

        let pab = &pam * &pmb;
        let qab = &qam * &qmb;
        let rab = &qmb * &ram + &pam * &rmb;
        (pab, qab, rab)
    }
}

fn chudnovsky(n: i64, scale: u64) -> BigDecimal {
    let (_p1n, q1n, r1n) = binary_split(1, n);

    // Calcola sqrt(10005) con precisione arbitraria
    //let scale = 1000u64;
    //let sqrt_c = BigDecimal::from_str("10005").unwrap();//.sqrt().unwrap();
    let sqrt_c = sqrt_bigdecimal(&BigDecimal::from(10005u64), scale);

    let big_426880 = BigDecimal::from(426880u64);
    let q1n_dec = BigDecimal::from(q1n.clone());
    let r1n_dec = BigDecimal::from(r1n);
    let term = BigDecimal::from(13591409i64) * q1n_dec.clone() + r1n_dec;

    (big_426880 * sqrt_c * q1n_dec) / term
}

fn main() {

    let digits = 1000000u64;

    let result = chudnovsky(chudnovsky_iterations_for_digits(1000) as i64, digits);
    println!("π ≈ {}", result);
}
