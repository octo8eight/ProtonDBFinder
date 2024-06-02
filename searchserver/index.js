const app = require("express")();
const Fuse = require("fuse.js");

// Game info class
class ProtonGame {
  constructor(steamDBInfo, protonDBInfo) {
    this.title = steamDBInfo.name;
    this.appID = steamDBInfo.appid;
    this.tier = protonDBInfo.tier[0].toUpperCase() + protonDBInfo.tier.slice(1); // Setting first letter of tier to uppercase
    this.highestTier =
      protonDBInfo.bestReportedTier[0].toUpperCase() +
      protonDBInfo.bestReportedTier.slice(1);
  }
}

// Search by name endpoint
app.get("/:query", async (req, res) => {
  res.setHeader("Content-Type", "application/json");
  let games;
  const query = req.params.query;

  // Fetching all games from SteamDB
  await fetch("https://api.steampowered.com/ISteamApps/GetAppList/v2/").then(
    async (response) => {
      const data = await response.json();
      games = data.applist.apps;
    },
  );

  // Searching nedeed game
  const fuse = new Fuse(games, { keys: ["name"] });
  const gameStartInfo = fuse.search(query)[0].item;
  if (gameStartInfo === undefined) {
    res.sendStatus(404);
  }

  // Fetching data from ProtonDB
  await fetch(
    `https://www.protondb.com/api/v1/reports/summaries/${gameStartInfo.appid}.json`,
  )
    .then(async (response) => {
      const data = await response.json();
      const game = new ProtonGame(gameStartInfo, data);
      res.send(game);
    })
    .catch(() => {
      res.sendStatus(404);
    });
});

// Search by id endpoint
app.get("/id/:query", async (req, res) => {
  res.setHeader("Content-Type", "application/json");
  let games;
  const query = req.params.query;

  // Fetching all games from SteamDB
  await fetch("https://api.steampowered.com/ISteamApps/GetAppList/v2/").then(
    async (response) => {
      const data = await response.json();
      games = data.applist.apps;
    },
  );

  // Searching nedeed game
  const fuse = new Fuse(games, { keys: ["appid"] });
  const gameStartInfo = fuse.search(query)[0].item;
  if (gameStartInfo === undefined) {
    res.sendStatus(404);
  }

  // Fetching data from ProtonDB
  await fetch(
    `https://www.protondb.com/api/v1/reports/summaries/${gameStartInfo.appid}.json`,
  )
    .then(async (response) => {
      const data = await response.json();
      const game = new ProtonGame(gameStartInfo, data);
      res.send(game);
    })
    .catch(() => {
      res.sendStatus(404);
    });
});

app.listen(3000);
