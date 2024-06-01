const app = require("express")();
const Fuse = require("fuse.js");

// Search endpoint
app.get("/:query", async (req, res) => {
  let games;
  const query = req.params.query;
  // Fetch all games from unofficial protondb API
  await fetch("https://protondb.max-p.me/games/").then(async (response) => {
    const data = await response.json();
    games = data;
  });

  // Searching nedeed game
  const fuse = new Fuse(games, { keys: ["title"] });
  const game = fuse.search(query)[0];

  // Checking if there are some games
  if (game !== undefined) {
    res.send(game.item);
  } else {
    res.sendStatus(404);
  }
});

// Getting name from appid
app.get("/name/:id", async (req, res) => {
  let games;
  const id = req.params.id;
  // Fetch all games from unofficial protondb API
  await fetch("https://protondb.max-p.me/games/").then(async (response) => {
    const data = await response.json();
    games = data;
  });

  // Searching nedeed game
  const fuse = new Fuse(games, { keys: ["appId"] });
  const game = fuse.search(id)[0];

  // Checking if there are some games
  if (game !== undefined) {
    res.send(game.item);
  } else {
    res.sendStatus(404);
  }
});

app.listen(3000);
