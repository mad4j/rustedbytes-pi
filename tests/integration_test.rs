
#[cfg(test)]
mod tests {
    use rustedbytes_pi::compute_pi;


    fn get_pi_digits() -> String {
        include_str!("pi_digits.txt").to_string()
    }

    #[test]
    fn test_one_million_digits() {

        let digits = 150;
        let p = compute_pi(digits);
        assert_eq!(p.to_string().len(), digits as usize + 2);
        
        for (i, c) in p.to_string().chars().enumerate() {
            let p = get_pi_digits().chars().nth(i).unwrap();
            assert_eq!(c, p, "Mismatch at index {}", i);
        }
    }
}