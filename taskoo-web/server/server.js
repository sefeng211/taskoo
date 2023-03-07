import { run, passStringToWASM, listEndpoint, addEndPoint, agendaEndpoint} from "./index.mjs";
import express from "express";
import cors from "cors";

const app = express();

app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.use(cors());

const server = app.listen(7000, () => {
  console.log(`Express running → PORT ${server.address().port}`);
});

app.get('/today', (req, res) => {
  let tasks = JSON.parse(run());
  res.send(tasks[0][1]);
});

app.post('/list', (req, res) => {
  const data = listEndpoint(req.body.data);
  let ret;
  try {
    ret = JSON.parse(data);
  } catch(e) {
    ret = {"error": data};
  }
  res.send(ret);
});

app.post('/agenda', (req, res) => {
  const data = agendaEndpoint(req.body.data);
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
  console.log(req.body.data);
  passStringToWASM(req.body.data);
});

app.post('/add', createPost, (req, res) => {
  console.log("add endpoint");
  addEndPoint(req.body.data);
});
