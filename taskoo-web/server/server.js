import { run } from "./index.mjs";

import express from "express";
import cors from "cors";

const app = express();
app.use(cors());

const server = app.listen(7000, () => {
  console.log(`Express running → PORT ${server.address().port}`);
});


app.get('/today', (req, res) => {
  let tasks = JSON.parse(run());
  res.send(tasks[0][1]);
});
