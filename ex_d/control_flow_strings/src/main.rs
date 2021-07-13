#![allow(dead_code, unused_mut, unused_variables)]

fn main() {

    // std::env::args() -> returns arguments program started w/ via cmd line
    // skip(1) -> skips first argument run?
    // collect -> creates collection from arguments this case is vector
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        println!("{}", arg);
    }
}

fn sum() {

}

fn double() {

}

fn count(arg: String) {

}
