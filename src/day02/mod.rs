use std::collections::HashMap;
use crate::day02::Colour::{Blue, Green, Red};

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq)]
enum Colour {
    Red,
    Blue,
    Green
}

fn check_game(draws: Vec<&str>, maxes: &HashMap<Colour, u64>) -> (HashMap<Colour, u64>, bool) {
    let mut possible: bool = true;
    let mut minimums: HashMap<Colour, u64> = HashMap::new();
    minimums.insert(Red, 0);
    minimums.insert(Blue, 0);
    minimums.insert(Green, 0);

    for draw in draws {
        let cubes: Vec<&str> = draw.split(", ").collect();
        for cube in cubes {
            let parts: Vec<&str> = cube.split(" ").collect();
            let amount = parts[0].parse::<u64>().unwrap();
            let colour: Colour;
            match parts[1] {
                "red" => colour = Red,
                "green" => colour = Green,
                "blue" => colour = Blue,
                _ => panic!("Unknown colour {}", parts[1])
            }
            if amount > *maxes.get(&colour).unwrap() {
                possible = false;
            }
            if amount > *minimums.get(&colour).unwrap() {
                minimums.insert(colour, amount);
            }
        }
    }

    (minimums, possible)
}

pub fn solve(task_input: Vec<String>) -> () {
    let mut total_possible: u64 = 0;
    let mut total_power: u64 = 0;

    let mut maxes: HashMap<Colour, u64> = HashMap::new();
    maxes.insert(Red, 12);
    maxes.insert(Green, 13);
    maxes.insert(Blue, 14);

    for line in task_input {
        let split: Vec<&str> = line.split(": ").collect();
        let game: Vec<&str> = split[0].split(" ").collect();
        let game_index: u64 = game[1].parse::<u64>().unwrap();

        let draws: Vec<&str> = split[1].split("; ").collect();
        let (minimums, possible) = check_game(draws, &maxes);
        if possible {
            total_possible += game_index;
        }
        let power = minimums.get(&Red).unwrap() * minimums.get(&Blue).unwrap() * minimums.get(&Green).unwrap();
        total_power += power;
    }

    println!("{}", total_possible);
    println!("{}", total_power);
}