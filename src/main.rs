use rand::Rng;
use std::io::{stdout, stdin, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;




fn main() {
    println!("Welcome to Rusty Dice!");
    println!("Lets shoot some dice!");

    //let mut user_input = String::new();
    let stdin = stdin();
    
    // this is from termion
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout,
            "{}{}Lets shoot some Dice!!{}Press 'q' to exit and 'y' to shoot the dice.{}",
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

}



fn shoot_dice() {
    let mut rng = rand::thread_rng();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let die_1 = rng.gen_range(1..7);
    let die_2 = rng.gen_range(1..7);
    let tot = die_1 + die_2;
    // output of user pressing the y key. 
    writeln!(stdout, "{}{}die 1 = {}{}die 2 = {}{}--------------{}Total: {}{}{}{}===================={}Press 'q' to quit and exit.{}Press 'y' to reshoot the dice.", 
             termion::clear::All,
             termion::cursor::Goto(1,1),
             die_1,
             termion::cursor::Goto(1,2),
             die_2,
             termion::cursor::Goto(1,3),
             termion::cursor::Goto(1,4),
             tot,
             termion::cursor::Goto(1,6,),
             if tot == 7 {
                "WOW 7! You WON!"
             }else{
                "Sorry, Roll again!"
             },
             termion::cursor::Goto(1,8,),
             termion::cursor::Goto(1,9),
             termion::cursor::Goto(1,10))
            .ok();

    stdout.flush().unwrap();

}
