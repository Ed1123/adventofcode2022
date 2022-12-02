fn main() {
    let input = include_str!("../input/day1.txt");
    let max_calories_elve: Option<u32> = input
        .split("\n\n")
        .map(|elve_foods| elve_foods.split("\n").flat_map(|x| x.parse::<u32>()).sum())
        .max();

    println!("{:?}", max_calories_elve)
}
