use radix_fmt::radix;

fn main() {
    let count = (1u32..1_000_000)
        .filter(|n: &u32| is_multibase_palindrome(*n))
        .count();

    println!("{}", count);
}

fn is_multibase_palindrome(n: u32) -> bool {
    let mut count = 0;
    for base in 2..11 {
        let string_repr = format!("{}", radix(n, base));
        if is_palindrome(&string_repr) {
            count += 1;
            if count == 2 {
                return true;
            }
        }
    }
    false
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
