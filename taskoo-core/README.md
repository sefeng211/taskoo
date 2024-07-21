# Taskoo Core
This directory contains the Taskoo source files that performs the actual
operations.

## Compilation
`taskoo-core` can be compiled standalone (though you need to use either
`taskoo-web` or `taskoo-cli` to use it).

To compile it for `taskoo-cli`, run
```
cargo build
```

To compile it for `taskoo-web`, run
```
./build_wasm.sh
```

# Installation
`rustup target add wasm32-wasi`

# Concepts
## Task
# Task Attributes
 - id (Unique)
 - body (Required)
 - priority
 - context (Required)
 - tags
 - date_created
 - date_due
 - date_scheduled
 - repetition_due
 - repetition_scheduled
 - state (Not customizable)
 - annotation
 - parent_task_ids (Unused)
## Context
## Tag
## State

