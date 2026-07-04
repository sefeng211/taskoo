# Taskoo
Yet another task management App, written in Rust.

## Introduction
This is the monorepo that contains three projects.

`taskoo-core`: Contains the code that interacts with the local database. Both `taskoo-cli` and `taskoo-web` requires this project to be compiled
to work. This enpowers taskoo to have multiple interfaces, while only need to main a single copy
of code for its backend.

`taskoo-cli`: It's the repo for the command line interface for taskoo. You don't need to build
`taskoo-web` if you only need the CLI version.

`tasko-web`: It's the repo for the web interface.


## Development
For day-to-day command-line usage and GTD workflow guidance, see
[CLI_USAGE.md](CLI_USAGE.md).

### Build the command-line version
#### taskoo-cli
```
make build-cli
```
This command compiles `taskoo-cli` along with `taskoo-core`, no need to compile `tasko-core` again.

#### To only build `taskoo-core`
```
make build-core
```
This command builds `taskoo-core` as a library, however this library can only be used for
`taskoo-cli`. If you need to build `taskoo-core` for the web interface, please see below.

### Build and run the web version
The web version has three pieces:

1. `taskoo-core`, compiled to WASI WebAssembly.
2. `taskoo-web/server`, a Node/Express backend that loads the WASM module.
3. `taskoo-web/webpack-ver`, the webpack frontend.

Install these prerequisites first:

- Rust with `rustup`
- Node.js and npm
- [wasi-sdk-16](https://github.com/WebAssembly/wasi-sdk/releases/tag/wasi-sdk-16)

Add the Rust WASI target:

```
rustup target add wasm32-wasip1
```

Set `WASI_SDK_PATH` to your wasi-sdk-16 installation. For example:

```
export WASI_SDK_PATH="$HOME/.local/share/wasi-sdk-16.0"
```

Build the WASM backend artifact:

```
make build-web
```

This copies `target/wasm32-wasip1/release/taskoo_core.wasm` into
`taskoo-web/server/taskoo_core.wasm`.

Install the Node dependencies:

```
cd taskoo-web/server && npm install
cd ../webpack-ver && npm install
```

Run both web processes from two terminals:

```
cd taskoo-web/server
npm run start
```

The backend listens on `http://localhost:7001`.

```
cd taskoo-web/webpack-ver
npm run start
```

The frontend listens on `http://localhost:4141`.

#### Database configuration
Taskoo reads its database path from:

```
~/.config/taskoo/config
```

If the config file does not exist, Taskoo creates a default database at:

```
~/.config/taskoo/tasks.db
```

To use an existing database, set `db_path` in `~/.config/taskoo/config`:

```
db_path=/absolute/path/to/tasks.db
```

The web server reads this config before starting WASI and preopens the configured database
directory automatically. If needed, you can override the paths when starting the server:

```
TASKOO_HOME=/path/to/home TASKOO_DB_DIR=/path/to/db/dir npm run start
```

### Run the web version with Docker Compose
The repository also includes a Docker Compose setup for the full web stack. From the
repo root:

```
docker compose up --build
```

The frontend is available at `http://localhost:4141`, and it proxies `/api` requests
to the backend container. The backend is also exposed directly at `http://localhost:7001`.

By default, Compose bind-mounts `${HOME}/taskoo_sync` into the backend container as
`/taskoo_sync` and writes a container config that points at `/taskoo_sync/tasks.db`.
To use a different sync directory:

```
TASKOO_SYNC_DIR=/absolute/path/to/taskoo_sync docker compose up --build
```

The named Docker volume `taskoo-data` is still used for the container's Taskoo home
and config directory.
