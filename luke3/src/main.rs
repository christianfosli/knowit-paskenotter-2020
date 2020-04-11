use radix_fmt::radix;

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
}
