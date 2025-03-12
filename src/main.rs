use std::time::{Duration, SystemTime};

use num_bigint::BigUint;

fn copy_start(num_str: &str, next_str: &str) -> usize {
    for (i, (a, b)) in num_str
        .bytes()
        .rev()
        .zip(next_str.bytes().rev())
        .enumerate()
    {
        if a != b {
            return next_str.len() - i;
        }
    }
    0
}

fn main() {
    let mut num = BigUint::from(5u8);
    let mut num_str = num.to_str_radix(10);
    let mut last = SystemTime::now();
    loop {
        let next = num.pow(2);
        let next_str = next.to_str_radix(10);

        let elapsed = last.elapsed().unwrap() > Duration::from_secs(1);
        if elapsed {
            last = SystemTime::now();
            println!("{num_str} ^ 2 = {next_str}");
        }

        let index = copy_start(&num_str, &next_str);
        if index == 0 {
            num_str = next_str;
            num = next;
        } else {
            num_str = next_str[index..].to_owned();
            num = num_str.parse().unwrap();
        }

        if elapsed {
            println!("the next number is {num_str}\n");
        }
    }
}
