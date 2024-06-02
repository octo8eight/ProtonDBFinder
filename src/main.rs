mod core;
use clap::Parser;
use core::args::ProtonDBFinderArgs;
use core::args::SubCommandType::{Name, ID};
use core::fetching::Game;
use std::panic;

#[tokio::main]
async fn main() {
    // Panic hook to handle panics and print panic information
    panic::set_hook(Box::new(|panic_info| {
        println!("{}", panic_info);
    }));

    // Collect command-line arguments
    let args = ProtonDBFinderArgs::parse();
    let game;
    match &args.sub_command_type {
        // Proccessing command-line arguments
        Name(_) => {
            // Retrieve game information by app name
            game = Game::get_by_name(args.get_game_name())
                .await
                .expect("ERROR: Invalid game name!");
        }
        ID(_) => {
            // Retrieve game information by appID
            game = Game::get_by_app_id(args.get_game_id())
                .await
                .expect("ERROR: Invalid appId!");
        }
        _ => panic!("Unexpected error!"),
    }

    println!("{}", game);
}
