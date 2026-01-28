// TUI App
mod blakcjack;
mod functions;

fn main() -> std::io::Result<()> {
    let mut user_input = String::from("new");

    loop {
        // Run game
        if user_input == String::from("new") {
            blakcjack::play();
        }

        // End of game options info
        println!("- Enter 'new' to start a new game");
        println!("- Enter 'exit' to close the game");
        println!("\n----------------------\n");
        print!("YOUR SELECTION => ");

        // Process user input
        user_input = functions::terminal_command(std::io::stdin());
        loop {
            if user_input.trim() == "new" || user_input.trim() == "exit" {
                break;
            } else {
                // Repeat input request
                print!("YOUR SELECTION => ");
                user_input = functions::terminal_command(std::io::stdin());
            }
        }

        // Final exit point
        if user_input.trim() == "exit" {
            break Ok(());
        }
    }
}
