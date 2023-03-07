# Taskoo-Web

This is the web interface for Taskoo.

## Contribute
The web version of Taskoo is split into two components. The server component
and the frontend component.

The server is a node server which runs queries to read information
from the Taskoo database. The frontend is the user interface which
communicate with the server.

Both components need to be run.

To run the server
```
cd server && npm run start
```
This will run a local web server listening port 7000.

To start the frontend
```

cd ui && npm run start
```
This will start the local webpack server.

