const app = require("express")();

const {
  fetchSteamDBData,
  searchGamebyID,
  searchGamebyName,
  fetchProtonDBData,
} = require("./core");

// Search by name endpoint
app.get("/:query", async (req, res) => {
  const query = req.params.query;

  // Fetching all games from SteamDB
  const games = await fetchSteamDBData();

  // Searching nedeed game and returning it if possible
  await searchGamebyName(res, games, query);
});

// Search by id endpoint
app.get("/id/:query", async (req, res) => {
  const query = req.params.query;

  // Fetching all games from SteamDB
  const games = await fetchSteamDBData();

  // Searching nedeed game and returning it if possible
  await searchGamebyID(res, games, query);
});

app.listen(3000);
