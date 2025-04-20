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

/// Compute the binary split for the Chudnovsky algorithm.
/// This function recursively computes the values of Pab, Qab, and Rab
/// for the given range [a, b].
/// The binary split method is used to optimize the computation.
pub fn binary_split(a: usize, b: usize) -> (IBig, IBig, IBig) {
    // Base case: if the range contains only one element
    if b - a == 1 {
        // Compute Pab, Qab, and Rab for the single element
        let pab =
            -(ibig!(6) * a - ibig!(5)) * (ibig!(2) * a - ibig!(1)) * (ibig!(6) * a - ibig!(1));
        let qab = ibig!(10939058860032000) * a.pow(3);
        let rab = &pab * (ibig!(545140134) * a + ibig!(13591409));

        // Return the computed values
        (pab, qab, rab)
    } else {
        // Recursive case: split the range into two halves
        let m = (a + b) / 2;

        // Use rayon for parallel computation to improve performance
        let ((pam, qam, ram), (pmb, qmb, rmb)) = rayon::join(
            || binary_split(a, m), // Compute for the left half
            || binary_split(m, b), // Compute for the right half
        );

        // Combine the results from the two halves
        let pab = &pam * &pmb;
        let qab = &qam * &qmb;
        let rab = &qmb * &ram + &pam * &rmb;

        // Return the combined results
        (pab, qab, rab)
    }
}

/// Compute the Chudnovsky algorithm for Pi with the specified number of iterations and digits.
pub fn chudnovsky(iterations: usize, digits: usize) -> FBig<Zero, 10> {
    // Ensure at least 2 iterations to get a valid result
    let iterations = iterations.max(2);

    println!("Iterations: {}", iterations);

    // More precision is needed in order to avoid error propagation
    let precision = digits + 2;

    let context = Context::<Zero>::new(precision);

    // Use rayon for parallel computation to improve performance
    let (q, (_p1n, q1n, r1n)) = rayon::join(
        || context.sqrt(dbig!(10005).repr()).value(),
        || binary_split(1, iterations),
    );

    // Perform division in parallel to further optimize
    let (n, d) = rayon::join(
        || ibig!(426880) * &q * &q1n,
        || ibig!(13591409) * &q1n + &r1n,
    );

    n / d
}

/// Calculate the number of iterations needed for the Chudnovsky algorithm
/// to achieve the desired number of digits.
pub fn chudnovsky_iterations(digits: usize) -> usize {
    // Each iteration gives approximately 14.181647462 decimal digits
    const DIGITS_PER_TERM: f64 = 14.181647462;
    ((digits as f64) / DIGITS_PER_TERM).ceil() as usize
}

/// Compute the digits of Pi using the Chudnovsky algorithm.
/// The result is a string representation of Pi with the specified number of digits.
pub fn compute_pi(digits: usize) -> String {
    // Compute the number of iterations needed for the Chudnovsky algorithm
    // to achieve the desired number of digits
    let iterations = chudnovsky_iterations(digits);

    // Compute pi using the Chudnovsky algorithm
    let pi = chudnovsky(iterations, digits);

    // Use a more efficient string slicing approach
    let pi_str = pi.to_string();
    pi_str.chars().take(digits + 2).collect()
}
