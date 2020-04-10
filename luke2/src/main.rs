use std::{cmp::Reverse, collections::BinaryHeap};

// Leylandtal er tal som kan skrivast på forma x^y + y^x for 1 < x <= y.
// Til dømes er 17 eit Laylandtal, for det kan skrivast som 3^2+2^3.
// Kva er summen av dei 250 minste leylandtala?

fn main() {
    let mut leylandtall = BinaryHeap::new();
    let mut x = 2;
    let mut y = 2;
    loop {
        let lt = calc_leylandtall(x, y);
        leylandtall.push(Reverse(lt));

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
                leylandtall.len()
            );
            break;
        }
    }

    let mut sum = 0;
    for _ in 0..250 {
        let smallest = leylandtall.pop().unwrap().0;
        sum += smallest;
    }
    println!("sum: {}", sum);
}

fn calc_leylandtall(x: u128, y: u128) -> u128 {
    x.pow(y as u32) + y.pow(x as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leylandtall_given17_shouldbetrue() {
        assert_eq!(17, calc_leylandtall(3, 2))
    }
}
