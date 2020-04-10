use std::collections::BinaryHeap;

fn main() {
    let mut laylandtall = BinaryHeap::new();
    let mut x = 2;
    let mut y = 2;
    loop {
        let lt = calc_laylandtall(x, y);
        if laylandtall.len() < 250 {
            println!(
                "adding number {}: {} with x {}, y {}",
                laylandtall.len() + 1,
                lt,
                x,
                y
            );
            laylandtall.push(lt);
        } else if lt < *laylandtall.peek().unwrap() {
            println!("adding extra {}, {}: {}", x, y, lt);
            laylandtall.push(lt);
        }

        if laylandtall.len() == 1_000_000 {
            break;
        }

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

    println!("getting the sum");
    for n in laylandtall.iter().rev().take(5) {
        println!("{}", n);
    }
    let sum: u128 = laylandtall.iter().rev().take(250).map(|n| n).sum();
    println!("{}", sum);
}

fn calc_laylandtall(x: u128, y: u128) -> u128 {
    println!("calculating x: {}, y: {}", x, y);
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
