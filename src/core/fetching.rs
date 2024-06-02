use crate::core::rating::colorize_rating;
use colored::*;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

// Define a struct to represent a game
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    title: Option<String>,
    appID: i32,
    tier: Option<String>,
    highestTier: Option<String>,
}

impl Game {
    // Function to retrieve game information by name from search-server
    pub async fn get_by_name(name: String) -> Result<Self, ExitFailure> {
        let url = format!("https://protondbserver.vercel.app/{}", name);
        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.text().await?;
        let game: Game = serde_json::from_str(&res)?;
        Ok(game)
    }

    // Function to retrieve game information by app ID from search-server
    pub async fn get_by_app_id(app_id: i32) -> Result<Self, ExitFailure> {
        let url = format!("https://protondbserver.vercel.app/id/{}", app_id);
        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.text().await?;
        let game: Game = serde_json::from_str(&res)?;
        Ok(game)
    }
}

impl std::fmt::Display for Game {
    // Function to format the Game struct for printing
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}\n{}: {}\n{}: {}\n{}: {}",
            "Name".bold(),
            self.title.as_ref().unwrap(),
            "AppID".bold(),
            self.appID,
            "Current rating".bold(),
            colorize_rating(self.tier.as_ref().unwrap()),
            "Highest rating".bold(),
            colorize_rating(self.highestTier.as_ref().unwrap()),
        )
    }
}
