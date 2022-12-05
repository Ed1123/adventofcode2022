use std::collections::HashSet;

fn decode_char(c: &char) -> u32 {
    let value = *c as u32;
    if c.is_uppercase() {
        return value - 38;
    } else {
        return value - 96;
    }
}

fn main() {
    let input_text = include_str!("../input/day3_1");
    let r: u32 = input_text
        .split('\n')
        .map(|line| {
            let half = line.len() / 2;
            let (a, b) = line.split_at(half);
            // println!("{:?}", (a, b));
            let a = a.chars().collect::<HashSet<_>>();
            let b = b.chars().collect::<HashSet<_>>();
            let common_item = match a.intersection(&b).next() {
                Some(x) => x,
                None => panic!("No item in common."),
            };
            // println!("{:?}", common_item);
            return decode_char(common_item);
        })
        .sum();
    println!("{:?}", r);
}
