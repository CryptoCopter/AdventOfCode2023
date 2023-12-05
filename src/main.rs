#![allow(non_snake_case)]

mod day01;

use structopt::StructOpt;

fn load_input(path: String) -> Vec<String> {
    let mut content = Vec::new();

    for line in std::fs::read_to_string(path).unwrap().lines() {
        content.push(line.to_string())
    }

    content
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Workshop", about = "Santa's workshop")]
struct Opt {
    day: u8,

    input_path: String,
}

fn main() {
    let opt = Opt::from_args();
    let task_input = load_input(opt.input_path);

    match opt.day {
        1 => println!("{}", day01::solve(task_input)),
        _ => std::process::exit(1)
    }
}
