use dashu::integer::UBig;
use dashu_float::DBig;
use dashu_base::SquareRoot;


/*

def binary_split(a, b):
    if b == a + 1:
        Pab = -(6*a - 5)*(2*a - 1)*(6*a - 1)
        Qab = 10939058860032000 * a**3
        Rab = Pab * (545140134*a + 13591409)
    else:
        m = (a + b) // 2
        Pam, Qam, Ram = binary_split(a, m)
        Pmb, Qmb, Rmb = binary_split(m, b)
        
        Pab = Pam * Pmb
        Qab = Qam * Qmb
        Rab = Qmb * Ram + Pam * Rmb
    return Pab, Qab, Rab

def chudnovsky(n):
    """Chudnovsky algorithm."""
    P1n, Q1n, R1n = binary_split(1, n)
    return (426880 * decimal.Decimal(10005).sqrt() * Q1n) / (13591409*Q1n + R1n)
*/


fn chudnovsky(n: usize) -> DBig {
    let (P1n, Q1n, R1n) = binary_split(1, n);
    let sqrt_10005 = DBig::from(10005).sqrt();
    let numerator = DBig::from(426880) * sqrt_10005 * &Q1n;
    let denominator = DBig::from(13591409) * &Q1n + &R1n;
    numerator / denominator
}

fn binary_split(a: usize, b: usize) -> (DBig, DBig, DBig) {
    if b == a + 1 {
        let Pab = DBig::from(-(6 * a as isize - 5) * (2 * a as isize - 1) * (6 * a as isize - 1));
        let Qab = DBig::from(10939058860032000u128) * UBig::from(a).pow(3);
        let Rab = &Pab * (545140134 * a + 13591409);
        (Pab, Qab, Rab)
    } else {
        let m = (a + b) / 2;
        let (Pam, Qam, Ram) = binary_split(a, m);
        let (Pmb, Qmb, Rmb) = binary_split(m, b);

        let Pab = &Pam * &Pmb;
        let Qab = &Qam * &Qmb;
        let Rab = &Qmb * &Ram + &Pam * &Rmb;
        (Pab, Qab, Rab)
    }
}

pub fn compute_pi(digits: usize) -> DBig {
    let n = (digits as f64 / 14.181647462725477_f64).ceil() as usize;
    let pi = chudnovsky(n);
    pi
}
