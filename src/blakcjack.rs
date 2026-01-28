// Backend Game Logic
use crate::functions;
use functions::{
    Card, build_deck, deal_hand, format_output, print_hand, score_hand, terminal_command,
};

pub fn play() {
    println!("{}", format_output("      BLACKJACK"));

    // Setup game
    let dealer_max = 18;
    let n_decks = 3;

    let mut deck = build_deck(n_decks);
    println!(" | {n_decks} decks in the mix & dealer plays to {dealer_max} |\n");

    let mut player: Vec<Card> = vec![];
    let mut dealer: Vec<Card> = vec![];

    let mut player_command = String::from("");

    loop {
        // Deal cards
        let mut deal = deal_hand(deck);
        deck = deal.0;

        // Complete setup before playing round(s)
        if player.len() < 2 && dealer.len() < 2 {
            player.append(&mut vec![deal.1.pop().expect("deal fail")]);
            dealer.append(&mut vec![deal.1.pop().expect("deal fail")]);

            continue;
        }

        // Update player hand
        if player_command == String::from("hit") {
            player.append(&mut vec![deal.1.pop().expect("deal fail")]);
        }

        // Update dealer hand
        if player_command == String::from("hold") && score_hand(&dealer) < dealer_max {
            dealer.append(&mut vec![deal.1.pop().expect("deal fail")]);
        }

        // Score hands
        let player_score = score_hand(&player);
        let dealer_score = score_hand(&dealer);

        // Player & dealer initial hand display
        if player.len() == 2 && dealer.len() == 2 {
            print_hand("dealer", &dealer, dealer_score);
            println!("");
            print_hand("player", &player, player_score);

            // Handle blackjack hands
            if player_score == 21 && dealer_score == 21 {
                println!("{}", format_output("BLACKJACK PUSH ( PLAYER & DEALER )"));
                break;
            } else if player_score == 21 {
                println!("{}", format_output("BLACKJACK ( PLAYER )"));
                break;
            } else if dealer_score == 21 {
                println!("{}", format_output("BLACKJACK ( DEALER )"));
                break;
            }
        } else if player_command == String::from("hold") {
            print_hand("dealer", &dealer, dealer_score);
        } else if player_command != String::from("hold") {
            print_hand("player", &player, player_score);
        }

        // Handle busted hands
        if player_score > 21 {
            println!("{}", format_output("BUSTED ( PLAYER )"));
            break;
        } else if dealer_score > 21 {
            println!("{}", format_output("WINNER ( DEALER BUSTED )"));
            break;
        }

        // Player rounds
        if player_command == "exit" {
            println!("{}", format_output("GAME ENDED BY PLAYER"));
            break;
        }
        if player_command != "hold" {
            // Loop user input until valid
            loop {
                print!("\nYOUR MOVE => ");
                player_command = terminal_command(std::io::stdin());

                if player_command == "hit" || player_command == "hold" || player_command == "exit" {
                    break;
                } else {
                    println!(
                        "{}",
                        format_output("INVALD USER INPUT : PLEASE ENTER 'HIT' or 'HOLD' or 'EXIT'")
                    );
                }
            }

            // if player holds, move on to dealer game, otherwise loop player :
            if player_command == "hit" {
                continue;
            }
        }

        // Dealer rounds
        if dealer_score < dealer_max {
            continue;
        }

        // Evaluate winner
        if player_score > dealer_score {
            println!("{}", format_output("WINNER ( PLAYER )"));
            break;
        } else if dealer_score > player_score {
            println!("{}", format_output("WINNER ( DEALER )"));
            break;
        } else {
            println!("{}", format_output("PUSH ( EQUAL SCORE )"));
            break;
        }
    }
}
