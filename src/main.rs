pub mod protondb_core;
use crate::protondb_core::Game;
use std::env;
use std::panic;

#[tokio::main]
async fn main() {
    // Panic hook to handle panics and print panic information
    panic::set_hook(Box::new(|panic_info| {
        println!("{}", panic_info);
    }));

    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    let app_name: String;

    // Check if at least one argument is provided
    if args.len() < 2 {
        panic!("ERROR: You didn't provided any args!");
    } else {
        app_name = args[1..].join(" ");
    }

    // Retrieve game information by app name
    let response = Game::get_by_name(app_name)
        .await
        .expect("ERROR: Invalid appId!");

    // Extract the first game from the response
    let game = response.get(0).expect("ERROR: Invalid name!");

    println!("{}", game);
}
