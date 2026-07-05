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

function parseEndpointData(data) {
  try {
    const parsed = JSON.parse(data);
    if (parsed && parsed.error) {
      return {status: 400, body: parsed};
    }
    return {status: 200, body: parsed};
  } catch(e) {
    return {status: 400, body: {"error": data}};
  }
}

app.get('/today', (req, res) => {
  let tasks = JSON.parse(run());
  res.send(tasks[0][1]);
});

app.post('/list', (req, res) => {
  console.log("Taskoo server: list endpoint");
  const data = Endpoints.List(req.body.data);
  console.log("Taskoo server: list endpoint, got data");
  console.log(data);
  const ret = parseEndpointData(data);
  res.status(ret.status).send(ret.body);
});

app.post('/agenda', (req, res) => {
  console.log("Taskoo server: agenda endpoint");
  const data = Endpoints.Agenda(req.body.data);
  console.log(data);
  const ret = parseEndpointData(data);
  res.status(ret.status).send(ret.body);
});

app.get('/metadata', (req, res) => {
  const ret = parseEndpointData(Endpoints.Metadata());
  res.status(ret.status).send(ret.body);
});

app.post('/info', (req, res) => {
  console.log("info endpoint");
  const ret = parseEndpointData(Endpoints.Info(req.body.data));
  res.status(ret.status).send(ret.body);
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
  const ret = parseEndpointData(Endpoints.Add(req.body.data));
  res.status(ret.status).send(ret.body);
});

app.post('/body', createPost, (req, res) => {
  console.log("body endpoint");
  const ret = parseEndpointData(Endpoints.Body(req.body.data));
  res.status(ret.status).send(ret.body);
});

app.post('/annotation', createPost, (req, res) => {
  console.log("annotation endpoint");
  const ret = parseEndpointData(Endpoints.Annotation(req.body.data));
  res.status(ret.status).send(ret.body);
});

app.post('/state_change', createPost, (req, res) => {
  console.log("state_change endpoint");
  const ret = parseEndpointData(Endpoints.StateChange(req.body.data));
  res.status(ret.status).send(ret.body);
});

app.post('/modify', createPost, (req, res) => {
  console.log("modify endpoint");
  const ret = parseEndpointData(Endpoints.Modify(req.body.data));
  res.status(ret.status).send(ret.body);
});

app.post('/delete', createPost, (req, res) => {
  console.log("delete endpoint");
  const ret = parseEndpointData(Endpoints.Delete(req.body.data));
  res.status(ret.status).send(ret.body);
});
