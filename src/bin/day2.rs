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
    Lose,
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
        Result::Lose
    }
}

lazy_static! {
    static ref POINTS_MAP: HashMap<Result, u32> = {
        let points_map = HashMap::from([(Result::Win, 6), (Result::Draw, 3), (Result::Lose, 0)]);
        points_map
    };
    static ref SHAPE_POINTS_MAP: HashMap<Shape, u32> = {
        let shape_points_map =
            HashMap::from([(Shape::Rock, 1), (Shape::Paper, 2), (Shape::Scissor, 3)]);
        shape_points_map
    };
}

fn calc_points(shape1: Shape, result: Result) -> u32 {
    let shape2 = match result {
        Result::Lose => wins(shape1),
        Result::Draw => shape1,
        Result::Win => wins(wins(shape1)),
    };
    let mut points = SHAPE_POINTS_MAP[&shape2];
    // println!("{}", points);
    let result = compare_shapes(shape2, shape1);
    // println!("{:?}", result);
    points = points + POINTS_MAP[&result];
    points
}

#[derive(Debug)]
struct Round {
    shape: Shape,
    result: Result,
}

fn main() {
    let decipher_col_1 = HashMap::from([
        ("A", Shape::Rock),
        ("B", Shape::Paper),
        ("C", Shape::Scissor),
    ]);
    let decipher_col_2 =
        HashMap::from([("X", Result::Lose), ("Y", Result::Draw), ("Z", Result::Win)]);

    let input = include_str!("../input/day2.txt");
    let rounds: Vec<Round> = input
        .split('\n')
        .map(|x| {
            let row: Vec<&str> = x.split(' ').collect();
            // println!({}, row[0])
            Round {
                shape: decipher_col_1[row[0]],
                result: decipher_col_2[row[1]],
            }
        })
        .collect();

    let result: u32 = rounds
        .iter()
        .map(|round| calc_points(round.shape, round.result))
        .sum();

    println!("{:?}", result)
}
