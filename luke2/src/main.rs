fn main() {
    let mut n = 2;
    let mut laylandtall: Vec<u32> = Vec::with_capacity(250);
    loop {
        if is_laylandtall(n) {
            laylandtall.push(n);
        }

        if laylandtall.len() == 250 {
            break;
        }

        n += 1;
    }

    let sum: u32 = laylandtall.iter().sum();
    println!("{}", sum);
}

fn is_laylandtall(n: u32) -> bool {
    for x in 2..n {
        for y in x..n {
            if x.pow(y) + y.pow(x) == n {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn laylandtall_given17_shouldbetrue() {
        assert!(is_laylandtall(17));
    }
}
