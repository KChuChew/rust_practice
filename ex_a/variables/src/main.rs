const STARTING_MISSILES : i32 = 8;
const READY_AMOUNT : i32 = 6;

fn main() {
    let (mut missiles, ready) : (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);

    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles -= ready;

    println!("I now have {} missiles left...", missiles);
}
