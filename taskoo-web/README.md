# Taskoo-Web

This is the web interface for Taskoo.

## Contribute
The web version of Taskoo is split into two components. The server component
and the frontend component.

The server is a node server which runs queries to read information
from the Taskoo database. The frontend is the user interface which
communicate with the server.

Both components need to be run.

## Fresh install
From a fresh clone, install:

- Rust with `rustup`
- Node.js and npm
- [wasi-sdk-16](https://github.com/WebAssembly/wasi-sdk/releases/tag/wasi-sdk-16)

Add the Rust WASI target:

```
rustup target add wasm32-wasip1
```

Set `WASI_SDK_PATH` to the wasi-sdk-16 installation:

```
export WASI_SDK_PATH="$HOME/.local/share/wasi-sdk-16.0"
```

From the repo root, build the WASM file used by the Node server:

```
make build-web
```

Install dependencies:

```
cd taskoo-web/server && npm install
cd ../webpack-ver && npm install
```

Run the backend from one terminal:

```
cd taskoo-web/server
npm run start
```

The backend listens on `http://localhost:7001`.

Run the frontend from another terminal:

```
cd taskoo-web/webpack-ver
npm run start
```

The frontend listens on `http://localhost:4141`.

## Database
Taskoo uses `~/.config/taskoo/config` to find the database:

```
db_path=/absolute/path/to/tasks.db
```

If the config file does not exist, Taskoo creates `~/.config/taskoo/tasks.db`.

The web server preopens the configured database directory for the WASI module. If the
database lives somewhere unusual, you can also pass it explicitly:

```
TASKOO_DB_DIR=/absolute/path/to/db/dir npm run start
```
