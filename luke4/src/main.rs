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
    let mut previous = 'å';
    s.chars().all(|c| {
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
    fn in_alphabetic_order_should_be_true() {
        assert!(in_alphabetic_order("demo"));
    }

    #[test]
    fn in_backwards_alphabetic_order_should_be_false() {
        assert!(in_backwards_alphabetic_order("trona"));
    }
}
