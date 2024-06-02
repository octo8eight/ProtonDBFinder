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

// Fetching all games from SteamDB
async function fetchSteamDBData() {
  return await fetch(
    "https://api.steampowered.com/ISteamApps/GetAppList/v2/",
  ).then(async (response) => {
    const data = await response.json();
    return data.applist.apps;
  });
}

async function fetchProtonDBData(res, gameStartInfo) {
  res.setHeader("Content-Type", "application/json");
  await fetch(
    `https://www.protondb.com/api/v1/reports/summaries/${gameStartInfo.appid}.json`,
    { headers: { "Content-Type": "application/json" } },
  )
    .then(async (response) => {
      const data = await response.json();
      const game = new ProtonGame(gameStartInfo, data);
      res.send(game);
    })
    .catch((e) => {
      console.log(e);
      res.sendStatus(404);
    });
}

async function searchGamebyName(res, games, query) {
  const fuse = new Fuse(games, { keys: ["name"] });
  const gameInfo = fuse.search(query)[0].item;
  if (gameInfo === undefined) {
    res.sendStatus(404);
  } else {
    await fetchProtonDBData(res, gameInfo);
  }
}

async function searchGamebyID(res, games, query) {
  const fuse = new Fuse(games, { keys: ["appid"] });
  const gameInfo = fuse.search(query)[0].item;
  if (gameInfo === undefined) {
    res.sendStatus(404);
  } else {
    await fetchProtonDBData(res, gameInfo);
  }
}

module.exports = {
  ProtonGame,
  fetchSteamDBData,
  searchGamebyID,
  searchGamebyName,
  fetchProtonDBData,
};
