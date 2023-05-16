use rand::Rng;
use std::io::{stdout, stdin, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;



fn main() {
    println!("Welcome to Rusty Dice!");
    println!("Ready to shoot some dice!");

    //score variable for user, keeps recored of total wins
    let mut user_score: i8 = 0;

    //let mut user_input = String::new();
    let stdin = stdin();
    
    // this is from termion
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,
            "{}{}Are you ready?!{}Press 'q' to exit and 'y' to shoot the dice.{}",
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
            Key::Char('y') => shoot_dice(&mut user_score),
            _ => {} 
        }
        stdout.flush().unwrap();
    }

    //cleans up screen.. i think.
    write!(stdout, "{}", termion::cursor::Show).unwrap();

    // says thankss
    exit(&user_score);
}



fn shoot_dice(score: &mut i8) {
    let mut rng = rand::thread_rng();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let die_1 = rng.gen_range(1..7);
    let die_2 = rng.gen_range(1..7);
    let tot = die_1 + die_2;

    if tot == 7{
        *score = *score + 1;
    }
    
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

fn exit(score: &i8) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    writeln!(stdout, "{}{}Thanks for playing!{}You won {} times.",
             termion::clear::All,
             termion::cursor::Goto(1,1),
             termion::cursor::Goto(1,2),
             score)
             .ok();

    stdout.flush().unwrap();
}
