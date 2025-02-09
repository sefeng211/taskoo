import { run, passStringToWASM, Endpoints} from "./index.mjs";

import express from "express";
import cors from "cors";

const app = express();

app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.use(cors());
app.use((req, res, next) => {
  res.header('Access-Control-Allow-Origin', '*'); // Allow from any origin
  res.header('Access-Control-Allow-Headers', 'Origin, X-Requested-With, Content-Type, Accept');
  next();
});

const ENDPOINTS = {};
const server = app.listen(7001, () => {
  console.log(`Express running → PORT ${server.address().port}`);
});

app.get('/today', (req, res) => {
  let tasks = JSON.parse(run());
  res.send(tasks[0][1]);
});

app.post('/list', (req, res) => {
  console.log("Taskoo server: list endpoint");
  const data = Endpoints.List(req.body.data);
  console.log("Taskoo server: list endpoint, got data");
  console.log(data);
  let ret;
  try {
    ret = JSON.parse(data);
    console.log("parsed success");
  } catch(e) {
    ret = {"error": data};
    console.log("parsed failed");
  }
  res.send(ret);
});

app.post('/agenda', (req, res) => {
  console.log("Taskoo server: agenda endpoint");
  const data = Endpoints.Agenda(req.body.data);
  console.log(data);
  let ret;
  try {
    ret = JSON.parse(data);
  } catch(e) {
    ret = {"error": data};
  }
  res.send(ret);
});

const createPost = (req, res, next) => {
    console.log('createPost', req.body)
    next()
}

app.post('/run', createPost, (req, res) => {
  console.log("/run endpoint");
  passStringToWASM(req.body.data);
});

app.post('/add', createPost, (req, res) => {
  console.log("add endpoint");
  Endpoints.Add(req.body.data);
});

app.post('/state_change', createPost, (req, res) => {
  console.log("state_change endpoint");
  Endpoints.StateChange(req.body.data);
});

app.post('/delete', createPost, (req, res) => {
  console.log("delete endpoint");
  Endpoints.Delete(req.body.data);
});
