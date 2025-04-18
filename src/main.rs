use dashu_float::{round::mode::HalfEven, Context, DBig};


fn main() {
    //let digits = 100;
    //let pi = compute_pi(digits);
    //println!("π con {digits} cifre:\n{:.100}", pi);

    let precision = 100;
    let context = Context::<HalfEven>::new(precision);

    let a = DBig::from(10005);

    let q = context.sqrt(a.repr());
    
    println!("sqrt(10005) = {}", q.value());
}