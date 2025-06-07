# miniNFS Server

A simple server implementation for `miniNFS` — serves files over a custom TCP protocol.

---

## Supported Commands

- `LIST` → list files in server directory
- `READ <filename>` → send file contents to client
- `WRITE <filename>` → receive file contents from client (multi-line input)
- `APPEND <filename>` → append data to file (multi-line input)
- `QUIT` → close connection

---

## Usage

### Build:

```bash
cargo build --release
```

### Run:

```bash
cargo run -- --file-path <path_to_serve> [--port <port>]
```

Example:

```bash
cargo run -- --file-path ./data --port 9090
```

If no `--port` is provided, defaults to `8080`.

---

## Command Line Arguments

| Argument              | Description                             | Default    |
| --------------------- | --------------------------------------- | ---------- |
| `--file-path` or `-f` | Path to the file/directory to be served | (required) |
| `--port` or `-p`      | Port to bind the server on              | `8080`     |

---

## Protocol Behavior

- Each client connects via TCP.
- The server supports multiple commands per persistent connection.
- `WRITE` and `APPEND` expect multi-line input terminated by `<EOF>`.
- Files are stored and served relative to the specified `file_path`.

---

## Example Architecture

```text
+-----------+         +------------+
| miniNFS   |  <-->   | miniNFS    |
| Client    |  TCP    | Server     |
+-----------+         +------------+
```

---

## Dependencies

- Rust 1.70+ recommended
- [clap](https://github.com/clap-rs/clap) for CLI argument parsing
