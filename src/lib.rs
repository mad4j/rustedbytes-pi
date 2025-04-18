use dashu::integer::IBig;
use dashu_float::round::mode::HalfAway;
use dashu_float::{Context, DBig};
use dashu_macros::{dbig, ibig};

/*
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


fn binary_split(a: usize, b: usize) -> (IBig, IBig, IBig) {

    if b == a+1 {
        let a = IBig::from(a);
        let pab =  -(ibig!(6) * &a - ibig!(5)) * (2 * &a - 1) * (6 * &a - 1);
        let qab = ibig!(10939058860032000) * &a.pow(3);
        let rab = &pab * (ibig!(13591409) + ibig!(545140134 )* &a);
        return (pab, qab, rab);
    } else {
        let m = (a + b) / 2;
        let (pam, qam, ram) = binary_split(a, m);
        let (pmb, qmb, rmb) = binary_split(m, b);

        let pab = &pam * &pmb;
        let qab = &qam * &qmb;
        let rab = &qmb * &ram + &pam * &rmb;
        return (pab, qab, rab);
    }
}

fn chudnovsky(iterations: usize, digits: usize) -> DBig {

    let precision = (digits as f64 * 1.2).ceil() as usize;
    let context = Context::<HalfAway>::new(precision);

    let q = context.sqrt(dbig!(10005).repr()).value();
    let (_p1n, q1n, r1n) = binary_split(1, iterations);

    let n = ibig!(426880) * &q * &q1n;
    let d = ibig!(13591409) * &q1n + &r1n;

    let r = n / d;

    println!("Pi: {} ", r);


    let r = r.with_precision(digits);
    
    println!("{:?}", r);

    r.value()
}

fn chudnovsky_iterations(digits: usize) -> usize {
    // Ogni iterazione dà circa 14.181647462 cifre decimali
    const DIGITS_PER_TERM: f64 = 14.181647462;
    ((digits as f64) / DIGITS_PER_TERM).ceil() as usize
}

pub fn compute_pi(digits: usize) -> DBig {
    let iterations = chudnovsky_iterations(digits);

    println!("Iterations: {}", iterations);
    println!("Digits: {}", digits);

    chudnovsky(iterations, digits)
}
