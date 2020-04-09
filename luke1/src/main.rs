use std::fs;

fn main() {
    let numbers = fs::read_to_string("numbers.txt").unwrap();
    let missing = find_missing_number(100_000, &numbers);
    println!("{}", missing);
}

fn find_missing_number(max: u32, num_str: &str) -> u32 {
    let nums: Vec<u32> = num_str.split(",").map(|n| n.parse().unwrap()).collect();
    for n in 1..max {
        if !nums.contains(&n) {
            return n;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_find_the_number() {
        let num_str = "1,2,3,5";
        let missing = find_missing_number(5, num_str);
        assert_eq!(4, missing);
    }
}
