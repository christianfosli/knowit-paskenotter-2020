use std::cmp::Ordering;
use std::fs;

fn main() {
    let words = fs::read_to_string("ord.txt").unwrap();
    let count = words
        .lines()
        .filter(|w| !in_alphabetic_order(&w) && !in_backwards_alphabetic_order(&w))
        .count();
    println!("{}", count);
}

fn in_alphabetic_order(s: &str) -> bool {
    let mut previous = 'a';
    s.chars().all(|c| {
        // å's ascii value is lower than æ and ø, thus we replace å with ù
        let c: char = if c == 'å' { 'ù' } else { c };
        let sorted = match previous.cmp(&c) {
            Ordering::Less => true,
            Ordering::Equal => true,
            Ordering::Greater => false,
        };
        previous = c;
        sorted
    })
}

fn in_backwards_alphabetic_order(s: &str) -> bool {
    let mut previous: char = 'ù'; // ø+1 in ascii
    s.chars().all(|c| {
        // å's ascii value is lower than æ and ø, thus we replace å with ù
        let c: char = if c == 'å' { 'ù' } else { c };
        let ordered = match previous.cmp(&c) {
            Ordering::Greater => true,
            Ordering::Equal => true,
            Ordering::Less => false,
        };
        previous = c;
        ordered
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_alphabetic_order() {
        assert!(in_alphabetic_order("demo"));
        assert!(in_alphabetic_order("bøø"));
        assert!(in_alphabetic_order("æøå"));

        assert_eq!(false, in_alphabetic_order("ål"));
        assert_eq!(false, in_alphabetic_order("øl"));
    }

    #[test]
    fn test_in_backwards_alphabetic_order() {
        assert!(in_backwards_alphabetic_order("trona"));
        assert!(in_backwards_alphabetic_order("øl"));
        assert!(in_backwards_alphabetic_order("åøæ"));

        assert_eq!(false, in_backwards_alphabetic_order("lå"));
        assert_eq!(false, in_backwards_alphabetic_order("lø"));
        assert_eq!(false, in_backwards_alphabetic_order("løa"));
    }
}
