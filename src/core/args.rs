use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ProtonDBFinderArgs {
    #[clap(subcommand)]
    pub sub_command_type: SubCommandType,
}

impl ProtonDBFinderArgs {
    pub fn get_game_name(&self) -> &String {
        self.sub_command_type.game_name()
    }
    pub fn get_game_id(&self) -> i32 {
        self.sub_command_type.game_id()
    }
}

#[derive(Debug, Subcommand)]
pub enum SubCommandType {
    /// Find game with name
    Name(NameCommand),
    /// Find game with appID
    ID(AppIDCommand),
}

impl SubCommandType {
    pub fn game_name(&self) -> &String {
        match self {
            Self::Name(arg) => &arg.game_name,
            _ => panic!("ERROR"),
        }
    }
    pub fn game_id(&self) -> i32 {
        match self {
            Self::ID(arg) => arg.game_id,
            _ => panic!("ERROR"),
        }
    }
}

#[derive(Debug, Args)]
pub struct NameCommand {
    /// The name of the game
    #[arg(num_args(0..))]
    pub game_name: String,
}

#[derive(Debug, Args)]
pub struct AppIDCommand {
    /// The appID of the game
    pub game_id: i32,
}
