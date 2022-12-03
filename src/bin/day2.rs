use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Result {
    Win,
    Loss,
    Draw,
}

fn wins(shape: Shape) -> Shape {
    match shape {
        Shape::Rock => Shape::Scissor,
        Shape::Scissor => Shape::Paper,
        Shape::Paper => Shape::Rock,
    }
}

fn compare_shapes(shape1: Shape, shape2: Shape) -> Result {
    if shape1 == shape2 {
        Result::Draw
    } else if wins(shape1) == shape2 {
        Result::Win
    } else {
        Result::Loss
    }
}

lazy_static! {
    static ref POINTS_MAP: HashMap<Result, u32> = {
        let points_map = HashMap::from([(Result::Win, 6), (Result::Draw, 3), (Result::Loss, 0)]);
        points_map
    };
    static ref SHAPE_POINTS_MAP: HashMap<Shape, u32> = {
        let shape_points_map =
            HashMap::from([(Shape::Rock, 1), (Shape::Paper, 2), (Shape::Scissor, 3)]);
        shape_points_map
    };
}

fn calc_points(shape1: Shape, shape2: Shape) -> u32 {
    let mut points = SHAPE_POINTS_MAP[&shape2];
    // println!("{}", points);
    let result = compare_shapes(shape2, shape1);
    // println!("{:?}", result);
    points = points + POINTS_MAP[&result];
    points
}

fn main() {
    let decipher = HashMap::from([
        ("A", Shape::Rock),
        ("B", Shape::Paper),
        ("C", Shape::Scissor),
        ("X", Shape::Rock),
        ("Y", Shape::Paper),
        ("Z", Shape::Scissor),
    ]);

    let input = include_str!("../input/day2.txt");
    let rounds: Vec<Vec<Shape>> = input
        .split('\n')
        .map(|x| x.split(' ').map(|x| decipher[x]).collect())
        .collect();

    let result: u32 = rounds
        .iter()
        .map(|shapes| calc_points(shapes[0], shapes[1]))
        .sum();

    println!("{:?}", result)
}
