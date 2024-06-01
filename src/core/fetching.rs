use crate::core::rating::colorize_rating;
use colored::*;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

// Define a struct to represent a game
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    id: i32,
    appId: i32,
    timestamp: i32,
    rating: Option<String>,
    notes: Option<String>,
    os: Option<String>,
    gpuDriver: Option<String>,
    specs: Option<String>,
    protonVersion: Option<String>,
    name: Option<String>,
}

// Define a struct to represent game information from search-server
#[derive(Serialize, Deserialize, Debug)]
struct GameInfo {
    appId: i32,
    title: Option<String>,
}

impl Game {
    // Function to retrieve game information by name from search-server
    pub async fn get_by_name(name: &String) -> Result<Vec<Self>, ExitFailure> {
        let url = format!("https://protondbserver.vercel.app/{}", name);
        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.text().await?;
        let body: GameInfo = serde_json::from_str(&res)?;
        let mut game = Self::get_by_app_id(body.appId).await?;
        let name = Game::get_app_name(body.appId).await?.title;

        game.iter_mut().for_each(|x| x.name = name.clone());
        Ok(game)
    }

    // Function to retrieve game details by app ID
    pub async fn get_by_app_id(app_id: i32) -> Result<Vec<Self>, ExitFailure> {
        let url = format!("https://protondb.max-p.me/games/{}/reports", app_id);
        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.text().await?;
        let mut body: Vec<Game> = serde_json::from_str(&res)?;
        let name = Game::get_app_name(app_id).await?.title;

        body.iter_mut().for_each(|x| x.name = name.clone());
        Ok(body)
    }

    // Function to retrieve game name
    async fn get_app_name(app_id: i32) -> Result<GameInfo, ExitFailure> {
        let url = format!("https://protondbserver.vercel.app/name/{}", app_id);
        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.text().await?;
        let game: GameInfo = serde_json::from_str(&res)?;
        Ok(game)
    }
}

impl std::fmt::Display for Game {
    // Function to format the Game struct for printing
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}",
            "Name".bold(),
            self.name.as_ref().unwrap(),
            "ID".bold(),
            self.appId,
            "Rating".bold(),
            colorize_rating(self.rating.as_ref().unwrap()),
            "OS".bold(),
            self.os.as_ref().unwrap().underline(),
            "Notes".bold(),
            self.notes.as_ref().unwrap().italic()
        )
    }
}
