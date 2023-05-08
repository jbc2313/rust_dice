use rand::Rng;
use std::io::{stdout, stdin, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;




fn main() {
    println!("Hello, world!");
    println!("Lets shoot some dice!");

    //let mut user_input = String::new();
    let stdin = stdin();
    
    // this is from termion
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout,
            "{}{}Lets shoot some Dice!!{}q to exit. y to shoot.{}",
            termion::clear::All,
            termion::cursor::Goto(1,1),
            termion::cursor::Goto(1,2),
            termion::cursor::Hide)
            .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(stdout,
               "{}{}",
               termion::cursor::Goto(1,1),
               termion::clear::CurrentLine)
                .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('y') => shoot_dice(),
            _ => {} 
        }
        stdout.flush().unwrap();

    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();

    





    // this is old code used before temion was added

    // let b1 = stdin.read_line(&mut user_input).unwrap();
    
    // if user_input == "no\n" {
    //    println!("you said no!");
    //    println!("I guess you dont like dice!");
    //    println!("BYE!");
    // }
    


    // setup for dice here 
    // if user_input == "yes\n" {
    

    // }

    // println!("=======================================");
    // println!("user input 'debug' = {:?}", user_input);
    // println!("user input was {} ", user_input);
    // println!("no of bytes read, {}", b1);
    //Ok(())
}

fn shoot_dice() {
    let mut rng = rand::thread_rng();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let die_1 = rng.gen_range(1..7);
    let die_2 = rng.gen_range(1..7);
    let tot = die_1 + die_2;
    // output of user pressing the y key. 
    writeln!(stdout, "{}{}die 1 === {}{}die 2 === {}{}--------------{}total = {}{}{}{}q to quit. y to reshoot.", 
             termion::clear::All,
             termion::cursor::Goto(1,1),
             die_1,
             termion::cursor::Goto(1,2),
             die_2,
             termion::cursor::Goto(1,3),
             termion::cursor::Goto(1,4),
             tot,
             termion::cursor::Goto(1,5,),
             if tot == 7 {
                "WOW 7! You WON!"
             }else{
                "Sorry, Roll again!"
             },
             termion::cursor::Goto(1,6))
            .ok();

    stdout.flush().unwrap();

}
