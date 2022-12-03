mod day01;
mod day02;
mod day03;

fn main() {
    let mut funcs: Vec<fn()> = Vec::new();
    funcs.push(day01::main);
    funcs.push(day02::main);
    funcs.push(day03::main);

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
