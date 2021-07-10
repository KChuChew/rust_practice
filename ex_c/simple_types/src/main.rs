#![allow(dead_code, unused_variables)]

fn main() {
    let coords : (f32, f32) = (6.3, 15.0);

    print_distance_x_y(coords.0, coords.1);
}

fn print_distance_x_y(x: f32, y: f32) {
    println!(
        "Distance from x: {}, y: {} to the origin is {}",
        x, y, (x.powf(2.0) + y.powf(2.0)).sqrt()
    );
}

fn print_distance_tuple(z : (f32, f32)) {
    println!(
        "Distance to the origin is {}", 
        (z.0.powf(2.0) + z.1.powf(2.0)).sqrt()
    );
}
