use rand::Rng;
use std::io::{stdout, stdin, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// command line program that allows you to roll 2 six-sided dice.
// you enter the number of dice you want to roll
// if you select 2 you play a game
// if you roll a 7, you win.
// after each roll, it displays what the two dice landed on
// 'y' will roll again, 'q' quits the game
// if you select anything other than 2, it is just a dice simulator/virtual dice

fn main() {

    // Welcome Message
    println!("Welcome to Rusty Dice!");
    println!("Get ready to shoot some dice!");

    //score variable for user, keeps recored of total wins
    let mut user_score: i8 = 0;
    let mut user_loses: i8 = 0;
    let mut num_user_dice: i8 = 0;

    //let mut user_input = String::new();
    let stdin1 = stdin();
    let stdin2 = stdin();
    // this is from termion
    let mut stdout = stdout().into_raw_mode().unwrap();


    // Get number of dice
    write!(stdout,
           "{}{}Please enter the number of dice you want to roll.{}",
           termion::clear::All,
           termion::cursor::Goto(1,1),
           termion::cursor::Hide,
          ).unwrap();
    stdout.flush().unwrap();

    num_user_dice = get_num_dice();


    // Start of Game
    write!(stdout,
           "{}{}You chose {} dice. Are you ready?!{}Press 'q' to exit and 'y' to shoot the dice.{}",
            termion::clear::All,
            termion::cursor::Goto(1,1),
            num_user_dice,
            termion::cursor::Goto(1,2),
            termion::cursor::Hide)
            .unwrap();
    stdout.flush().unwrap();

    // This is the main game loop
    for c in stdin2.keys() {
        write!(stdout,
               "{}{}",
               termion::cursor::Goto(1,1),
               termion::clear::CurrentLine)
               .unwrap();

        match c.unwrap() {
            Key::Char('q') => break, //breaks out of main game loop
            Key::Char('y') => if num_user_dice == 2 {shoot_two_dice(&mut user_score, &mut user_loses)}else{roll_dice(&num_user_dice)},
            _ => {}
        }
        stdout.flush().unwrap();
    }

    //cleans up screen.. i think.
    write!(stdout, "{}", termion::cursor::Show).unwrap();

    // says thankss
    exit(&user_score, &user_loses);
}


// this is the main game function
fn shoot_two_dice(score: &mut i8, loses: &mut i8) {
    let mut rng = rand::thread_rng();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let die_1 = rng.gen_range(1..7);
    let die_2 = rng.gen_range(1..7);
    let tot = die_1 + die_2;

    if tot == 7{
        *score = *score + 1;
    }else{
        *loses = *loses + 1;
    }

    // going to remove the smiling guy :(
    // output of user pressing the y key. 
    writeln!(stdout, "{}{}die 1 = {}{}die 2 = {} {}--------------{}Total: {}{}{}{}===================={}Press 'q' to quit and exit.{}Press 'y' to reshoot the dice.", 
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
                "7! You WIN!"
             }else{
                "Sorry, Roll again!"
             },
             termion::cursor::Goto(1,8,),
             termion::cursor::Goto(1,9),
             termion::cursor::Goto(1,10))
            .ok();

    stdout.flush().unwrap();

}


// function when user doesnt select 2 dice. This is just to roll the number of dice and get the sum of all dice.
// need to add note to end of writeln! that says y to roll again q to quit
fn roll_dice(num_dice: &i8) {
    let mut rng = rand::thread_rng();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut loop_size = *num_dice;
    let mut dice_vec: Vec<i8> = Vec::new();
    let mut dice_total: i8 = 0;
    while loop_size != 0 {
        let a = rng.gen_range(1..7);
        dice_vec.push(a);
        loop_size -= 1;
    }

    let dice_vec_iter = dice_vec.iter();
    for index in dice_vec_iter {
        dice_total = dice_total + index;
    }


    writeln!(stdout, "{}{} The sum of all {} dice is {}{}",
                    termion::clear::All,
                    termion::cursor::Goto(1,1),
                    num_dice,
                    dice_total,
                    termion::cursor::Hide)
                    .ok();

    stdout.flush().unwrap();

}


fn get_num_dice() -> i8 {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    let mut num_dice: i8 = 0;
    
    writeln!(stdout, "{}{}How many dice do you want to roll?{}",
             termion::clear::All,
             termion::cursor::Goto(1,1),
             termion::cursor::Hide).unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
            write!(stdout,
                "{}{}",
                termion::cursor::Goto(1,1),
                termion::clear::CurrentLine)
                    .unwrap();

            match c.unwrap() {
                Key::Char('1') => num_dice = 1,
                Key::Char('2') => num_dice = 2,
                Key::Char('3') => num_dice = 3,
                Key::Char('4') => num_dice = 4,
                _ => {println!("YOU ENTERED A BAD KEY");
                      continue;
                }
            }
            stdout.flush().unwrap();
            if num_dice > 0 {break};
    }

    return num_dice;
}


// this will be implemented later
// function to choose the number of sides on the die
fn get_sides_dice() -> i8 {
    let mut sides_dice: i8 = 6;

    return sides_dice;
}

// displays the total score at exit of game
fn exit(score: &i8, loses: &i8) {
    // Lets add total losses here too.
    let mut stdout = stdout().into_raw_mode().unwrap();
    writeln!(stdout, "{}{}Thanks for playing!{}You won {} times.{}You lost {} times.",
             termion::clear::All,
             termion::cursor::Goto(1,1),
             termion::cursor::Goto(1,2),
             score,
             termion::cursor::Goto(1,3),
             loses)
             .ok();

    stdout.flush().unwrap();
}
