#![allow(dead_code, unused_variables)]

fn main() {
    //let coords : (f32, f32) = (6.3, 15.0);

    //print_distance_x_y(coords.0, coords.1);
    //print_distance_tuple(coords);
    
    //let coords_arr : [f32; 2] = [coords.0, coords.1];

    //print_array(coords_arr);
    let mut series = [22,15,3,4,420,103,7,800];

    //print_even_vals_array_for(&series);
    print_even_vals_array_foreach(& mut series);
}

fn print_difference(x: f32, y: f32) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

fn print_array(a: [f32; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

fn print_even_vals_array_for(a:&[i32]) {
    let len = a.len();

    for i in 0..len {
        if a[i] & 0x1 == 1 {
            continue;
        }

        println!("Element {} of array is even val {}", i, a[i]);
    }
}

fn print_even_vals_array_foreach(a: &mut[i32]) {

    for num in a.iter_mut() {
        if *num & 0x1 == 1 {
            *num = 69;
            continue; 
        }
        
        *num = 420;
    }

    for num in a.iter() {
        println!("changed array vals {}", num);
    }
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
