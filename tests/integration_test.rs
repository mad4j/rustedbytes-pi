
#[cfg(test)]
mod tests {
    use rustedbytes_pi::{chudnovsky, chudnovsky_iterations_for_digits};


    fn get_pi_digits() -> String {
        include_str!("pi_digits.txt").to_string()
    }

    #[test]
    fn test_one_million_digits() {

        let p = chudnovsky(chudnovsky_iterations_for_digits(1_000_000 as usize) as i64, 1_000_000);
        assert_eq!(p.to_string(), get_pi_digits());
    }
}