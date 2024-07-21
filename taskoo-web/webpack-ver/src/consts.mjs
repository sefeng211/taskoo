const SERVER_ENDPOINT = "http://100.86.23.103:7001";

const SERVER_ENDPOINT_MAPPING = {
  agenda: SERVER_ENDPOINT + "/agenda",
  list: SERVER_ENDPOINT + "/list",
  add: SERVER_ENDPOINT + "/add",
  run: SERVER_ENDPOINT + "/run",
  today: SERVER_ENDPOINT + "/today",
  state_change: SERVER_ENDPOINT + "/state_change"
};

export {
  SERVER_ENDPOINT_MAPPING
};
