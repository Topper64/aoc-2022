mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

fn main() {
    let funcs: Vec<fn()> = vec![
        day01::main,
        day02::main,
        day03::main,
        day04::main,
        day05::main,
        day06::main,
        day07::main,
        day08::main,
        day09::main,
        day10::main,
        day11::main,
        day12::main,
        day13::main,
        day14::main,
    ];

    let mut args = std::env::args();
    args.next(); // Skip arg 0
    if let Some(arg) = args.next() {
        match arg
            .parse::<usize>()
            .ok()
            .and_then(|i| i.checked_sub(1))
            .and_then(|i| funcs.get(i))
        {
            Some(func) => func(),
            _ => println!("could not run {}", arg),
        }
    } else {
        for (i, func) in funcs.iter().enumerate() {
            println!("day {}", i + 1);
            func();
        }
    }
}
