fn main() {
    let sum: u32 = (1u32..10_000_000)
        .filter(|n| n.becomes_palindrome_after_42_iterations())
        .sum();
    println!("{}", sum);
}

trait PalindromeExtensions {
    fn reverse(self) -> u32;
    fn is_palindrome(self) -> bool;
    fn becomes_palindrome_after_42_iterations(self) -> bool;
}

impl PalindromeExtensions for u32 {
    fn reverse(self) -> u32 {
        let mut num = self;
        let mut rev = 0u32;
        while num > 0 {
            let last_digit = num % 10;
            rev = rev * 10 + last_digit;
            num = num / 10;
        }

        rev
    }
    fn is_palindrome(self) -> bool {
        self == self.reverse()
    }
    fn becomes_palindrome_after_42_iterations(self) -> bool {
        let mut iterations = 1;
        let mut n = self;
        loop {
            if iterations > 42 {
                return false;
            }
            if n.is_palindrome() {
                return match iterations {
                    42 => true,
                    _ => false,
                };
            }
            n += n.reverse();
            iterations += 1;
        }
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
    }

    #[test]
    fn test_palindrome() {
        assert!(202.is_palindrome());
        assert!(55.is_palindrome());
        assert_eq!(false, 123.is_palindrome());
    }
}
