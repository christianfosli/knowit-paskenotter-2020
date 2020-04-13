use rayon::prelude::*;

fn main() {
    let sum: u128 = (1u128..10_000_001)
        .collect::<Vec<u128>>()
        .par_iter()
        .filter(|n| n.becomes_palindrome_after_n_iterations(42))
        .sum();
    println!("{}", sum);
}

trait PalindromeExtensions {
    fn reverse(self) -> u128;
    fn is_palindrome(self) -> bool;
    fn becomes_palindrome_after_n_iterations(self, iterations: u8) -> bool;
}

impl PalindromeExtensions for u128 {
    fn reverse(self) -> u128 {
        format!("{}", self)
            .chars()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap()
    }
    fn is_palindrome(self) -> bool {
        self == self.reverse()
    }
    fn becomes_palindrome_after_n_iterations(self, iterations: u8) -> bool {
        let mut n = self;
        for i in 1..(iterations + 1) {
            n = n.checked_add(n.reverse()).expect(&format!(
                "overflow checking {} on iteration {} : {}",
                self, i, n
            ));
            if n.is_palindrome() {
                return i == iterations;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(42, 24.reverse());
        assert_eq!(999, 999.reverse());
        assert_eq!(12001, 10021.reverse());
        assert_eq!(011, 110.reverse());
    }

    #[test]
    fn test_palindrome() {
        assert!(202.is_palindrome());
        assert!(55.is_palindrome());
        assert!((110 + 110.reverse()).is_palindrome());
        assert_eq!(false, 123.is_palindrome());
    }

    #[test]
    fn test_64_should_be_palindrome_after_2_iterations() {
        assert!(64.becomes_palindrome_after_n_iterations(2));
    }

    #[test]
    fn test_561_should_be_palindrome_after_3_iterations() {
        assert_eq!(false, 561.becomes_palindrome_after_n_iterations(2));
        assert!(561.becomes_palindrome_after_n_iterations(3));
        assert_eq!(false, 561.becomes_palindrome_after_n_iterations(4));
    }
}
