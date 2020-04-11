use radix_fmt::radix;

// Problem: How many of 1..1_000_000 are palindromes in more than one numeric system,
// if we look at numeric systems from base 2 until base 10 inclusive?

fn main() {
    let count = (1u32..1_000_000)
        .filter(|n| is_multibase_palindrome(*n))
        .count();

    println!("{}", count);
}

fn is_multibase_palindrome(n: u32) -> bool {
    (2u8..11)
        .map(|base| format!("{}", radix(n, base)))
        .filter(|formatted| is_palindrome(formatted))
        .count()
        >= 2
}

fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_should_be_true() {
        assert!(is_palindrome("131"));
    }

    #[test]
    fn is_multibase_palindrome_should_be_true() {
        // 51 is 0b110011 and 303 base 4, and therefore a multibase palindrome
        assert!(is_multibase_palindrome(51));
    }
}
