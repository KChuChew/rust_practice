const STARTING_MISSILES : i32 = 8;
const READY_AMOUNT : i32 = 6;

fn main() {
    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;

    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles -= ready;

    println!("I now have {} missiles left...", missiles);
}
