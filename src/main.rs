use rand::Rng;
use std::io::stdin;

fn continue_playing() {
    let mut balance = 10.0;
    println!("You're starting off with ${}", balance);
    println!(" ");
    // The random reward the player can get
    balance -= 1.0;
    balance += rand_scrap();
    while balance >= 1.0 {
        // Asking if the player wants to continue
        println!("Your balance is ${}", balance);
        println!("Do you want to continue? (y/n)");

        // User's input
        let mut player_continue = String::new();
        stdin()
            .read_line(&mut player_continue)
            .expect("Failed to read line");

        // Removing the whitespace after entering the user input
        player_continue = player_continue.split_whitespace().collect();
        // Seeing if the player is going to continue
        if player_continue == "y" {
            balance -= 1.0;
            balance += rand_scrap();
        } else if player_continue == "n" {
            break;
        }
    }
    if balance < 1.0 {
        println!(" ");
        println!("Sorry you ran out of money");
    }
}

fn rand_scrap() -> f64 {
    let mut money_reward = 0.0;
    let rewards = ["rust", "iron", "steel"];
    let winnings = rewards[rand::thread_rng().gen_range(0..3)];
    println!("You won, {}", winnings);
    if winnings == "steel" {
        money_reward += 1.5;
    } else if winnings == "iron" {
        money_reward += 0.5;
    } else {
        money_reward = 0.0;
    }
    money_reward
}

fn main() {
    // First Welcome message
    println!("Hello there, please input your name:");

    // Inputing Player name
    let mut player_name = String::new();
    stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line");

    // Removing the whitespace after entering the user input
    player_name = player_name.split_whitespace().collect();
    // Welcoming the user with their name
    println!("Welcome {} to the Junkyard Hunt", player_name);
    println!("Each time you play it's $1");

    // While the person still has money ask if they want to continue
    continue_playing();

    // Good bye message after the continue_playing function is done
    println!("See you soon, {}", player_name);
}
