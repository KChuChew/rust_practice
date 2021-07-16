#![allow(unused_mut)]

fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    let is_arg_string_plural = is_string_plural(&arg);

    if is_arg_string_plural {
        println!("The argument string {} is plural!", arg);
    }
    else {
        println!("The argument string {} is NOT plural!", arg);
    }
    
    pluralize_string(& mut arg);
    println!("I have many {} ", arg);
    
    consume_string(arg);

    println!("Result of adding 420 and 215 = {}", add_two_ints_by_reference(&420, &215));
}

fn add_two_ints_by_reference(v1: &i32, v2: &i32) -> i32{
    (*v1) + (*v2)
}

fn consume_string(s: String) {
    if s.contains("abc") {
        println!("say your abcs");
        return; 
    }
    
    println!("goodbye with no abcs :(");
}

fn is_string_plural(s: &String) -> bool {
    if s.ends_with("s") {
        return true;
    }

    return false;
}

fn pluralize_string(s: & mut String) {
    if s.ends_with("s") == false {
        s.push_str("s");
    }
    else {
        s.push_str("es");
    }
}
