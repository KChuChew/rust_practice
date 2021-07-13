#![allow(dead_code, unused_mut, unused_variables)]

fn main() {

    // std::env::args() -> returns arguments program started w/ via cmd line
    // skip(1) -> skips first argument run?
    // collect -> creates collection from arguments this case is vector
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if arg == "sum" {
            sum(); 
            continue;
        }

        if arg == "double" {
            double();
            continue;
        }

        count(arg);
    }
}

fn sum() {
    let mut sum = 0; 
    for val in 7..=23 {
        sum += val;
    }

    println!("Sum of vals 7 -> 23 inclusive {}", sum);
    println!();
}

fn double() {
    let mut count = 0;
    let mut x = 1;

    while x <= 500 {
        x *= 2;
        count += 1;
    }

    println!("Double starting value 1 {} times to exceed val of 500", count);
}

fn count(arg: String) {
    let mut loop_count = 0;

    loop {
        if loop_count == 8 {
            break;
        }

        print!("{} ", arg);
        loop_count += 1; 
    }

    println!();
}
