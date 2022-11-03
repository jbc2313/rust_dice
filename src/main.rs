use rand::Rng;
use std::io;


fn main() -> io::Result<()> {
    println!("Hello, world!");
    println!("Lets shoot some dice!");

    let mut user_input = String::new();
    let stdin = io::stdin();
    let b1 = stdin.read_line(&mut user_input).unwrap();
    
    if user_input == "no\n" {
       println!("you said no!");
       println!("I guess you dont like dice!");
       println!("BYE!");
    }
    
    // setup for dice here
    
    let mut rng = rand::thread_rng();

    let die_1 = rng.gen_range(1..7);
    let die_2 = rng.gen_range(1..7);
    
    if user_input == "yes\n" {
        println!("die 1 === {}", die_1);
        println!("die 1 === {}", die_2);
    }

    println!("=======================================");
    println!("user input 'debug' = {:?}", user_input);
    println!("user input was {} ", user_input);
    println!("no of bytes read, {}", b1);
    Ok(())
}
