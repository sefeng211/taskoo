let SERVER_ENDPOINT = "http://localhost:7001";

if (window.location.hostname === "taskoo.seanfeng.dev") {
  SERVER_ENDPOINT = "";
}

const SERVER_ENDPOINT_MAPPING = {
  agenda: SERVER_ENDPOINT + "/agenda",
  list: SERVER_ENDPOINT + "/list",
  add: SERVER_ENDPOINT + "/add",
  run: SERVER_ENDPOINT + "/run",
  today: SERVER_ENDPOINT + "/today",
  state_change: SERVER_ENDPOINT + "/state_change",
  del: SERVER_ENDPOINT + "/delete"
};

export {
  SERVER_ENDPOINT_MAPPING
};
