use dashu::integer::IBig;
use dashu_float::round::mode::Zero;
use dashu_float::{Context, FBig};
use dashu_macros::{dbig, ibig};

/*
https://en.wikipedia.org/wiki/Chudnovsky_algorithm

#Note: For extreme calculations, other code can be used to run on a GPU, which is much faster than this.
import decimal

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


print(f"1 = {chudnovsky(2)}")  # 3.141592653589793238462643384

decimal.getcontext().prec = 100 # number of digits of decimal precision
for n in range(2,10):
    print(f"{n} = {chudnovsky(n)}")  # 3.14159265358979323846264338...
 */


pub fn binary_split(a: usize, b: usize) -> (IBig, IBig, IBig) {
    if b - a == 1 {
        let a = IBig::from(a);
        let pab = -(ibig!(6) * &a - ibig!(5)) * (ibig!(2) * &a - ibig!(1)) * (ibig!(6) * &a - ibig!(1));
        let qab = ibig!(10939058860032000) * a.pow(3);
        let rab = &pab * (ibig!(545140134) * &a + ibig!(13591409));
        (pab, qab, rab)
    } else {
        let m = (a + b) / 2;

        // Use rayon for parallel computation to improve performance
        let ((pam, qam, ram), (pmb, qmb, rmb)) = rayon::join(
            || binary_split(a, m),
            || binary_split(m, b),
        );

        let pab = &pam * &pmb;
        let qab = &qam * &qmb;
        let rab = &qmb * &ram + &pam * &rmb;
        (pab, qab, rab)
    }
}

pub fn chudnovsky(iterations: usize, digits: usize) -> FBig<Zero, 10> {

    // ensure at least 2 iterations to get a valid result
    let iterations = iterations.max(2);

    // more precision is needed in order to avoid error propagation
    let precision = digits+2;

    let context = Context::<Zero>::new(precision);

    let (q, (_p1n, q1n, r1n)) = rayon::join(
        || context.sqrt(dbig!(10005).repr()).value(), 
        || binary_split(1, iterations),
    );

    let n = ibig!(426880) * &q * &q1n;
    let d = ibig!(13591409) * &q1n + &r1n;

    let r = n / d;

    r
}

pub fn chudnovsky_iterations(digits: usize) -> usize {
    // Ogni iterazione dà circa 14.181647462 cifre decimali
    const DIGITS_PER_TERM: f64 = 14.181647462;
    ((digits as f64) / DIGITS_PER_TERM).ceil() as usize
}

pub fn compute_pi(digits: usize) -> String {

    let iterations = chudnovsky_iterations(digits);
    chudnovsky(iterations, digits).to_string()[..digits + 2].to_string()
}
