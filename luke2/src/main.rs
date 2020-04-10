use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let mut laylandtall = BinaryHeap::new();
    let mut x = 2;
    let mut y = 2;
    loop {
        let lt = calc_laylandtall(x, y);
        laylandtall.push(Reverse(lt));

        if (x).checked_pow((y + 1) as u32).is_some() {
            y += 1;
        } else if (x + 1).checked_pow((x + 1) as u32).is_some() {
            x += 1;
            y = x;
        } else {
            println!(
                "only overflows left.... exiting with x {}, y {}, count {}",
                x,
                y,
                laylandtall.len()
            );
            break;
        }
    }

    let mut sum = 0;
    for _ in 0..250 {
        let smallest = laylandtall.pop().unwrap().0;
        sum += smallest;
    }
    println!("sum: {}", sum);
}

fn calc_laylandtall(x: u128, y: u128) -> u128 {
    x.pow(y as u32) + y.pow(x as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn laylandtall_given17_shouldbetrue() {
        assert_eq!(17, calc_laylandtall(3, 2))
    }
}
