#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    
    for arg in args {
        
        if arg == "sum" {
            sum()
        } 
        else if arg == "double" {
            double()
        }
        else {
            count(arg)
        };
    }
}

fn sum() {
    let mut sum = 0;
    
    for i in 7..=23 {
        sum = sum + i;
    }


    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    
    while x < 500 {
        count += 1;
        x *= 2;
    }


    println!("You can double x {} times until x is larger than 500", count);
}

fn count(arg: String) {
    // You will need to count your loops, somehow.  Run it with `cargo run bananas`
    // Execute this line 8 times, and then break. `print!` doesn't add a newline.

    let mut count = 0;
    loop {
        if count == 8 {
            break
        };
        print!("{} ", arg);
        count += 1;
    }

    println!(); // This will output just a newline at the end for cleanliness.
}