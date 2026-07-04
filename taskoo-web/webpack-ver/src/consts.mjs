let SERVER_ENDPOINT = "/api";

const SERVER_ENDPOINT_MAPPING = {
  agenda: SERVER_ENDPOINT + "/agenda",
  list: SERVER_ENDPOINT + "/list",
  add: SERVER_ENDPOINT + "/add",
  modify: SERVER_ENDPOINT + "/modify",
  metadata: SERVER_ENDPOINT + "/metadata",
  run: SERVER_ENDPOINT + "/run",
  today: SERVER_ENDPOINT + "/today",
  state_change: SERVER_ENDPOINT + "/state_change",
  delete: SERVER_ENDPOINT + "/delete"
};

export {
  SERVER_ENDPOINT_MAPPING
};
