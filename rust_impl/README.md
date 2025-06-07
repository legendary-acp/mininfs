# miniNFS

A simple mini-Network File System built in Rust.

- Custom TCP-based protocol.
- Supports multiple commands over persistent connections.
- Designed as a learning project for file protocols and network programming.

---

## Project Structure

```text
miniNFS/
├── client/    # miniNFS client (command-line interactive client)
├── server/    # miniNFS server (serves files over TCP)
├── README.md  # Top-level project README
```

Each part has its own README.

---

## Features

- List files on server
- Read file contents
- Write new files
- Append to existing files
- Multi-command over single connection
- Simple CLI-based operation

---

## Building

### Prerequisites

- Rust 1.70+ recommended

### Build client:

```bash
cd client
cargo build --release
```

### Build server:

```bash
cd server
cargo build --release
```

---

## Running

### Run server:

```bash
cargo run -- --file-path ./data --port 9090
```

### Run client:

```bash
cargo run -- --server 127.0.0.1:9090
```

---

## Protocol Overview

- Commands are sent as lines:

```text
LIST
READ <filename>
WRITE <filename>
APPEND <filename>
QUIT
```

- For `WRITE` and `APPEND`, client sends multi-line input terminated by `<EOF>`.

```text
WRITE myfile.txt
line 1
line 2
<EOF>
```

- Server responds with:

```text
OK
<content>
END
```

or

```text
ERROR
<error message>
END
```

---

## Example Architecture

```text
+-----------+         +------------+
| miniNFS   |  <-->   | miniNFS    |
| Client    |  TCP    | Server     |
+-----------+         +------------+
```

---

## License

MIT License.

---

## Notes

This project is intentionally simple and meant as a learning exercise. It does **not implement** full NFS protocol compatibility.

Future improvements might include:

- Concurrent client handling (multi-threaded server)
- Partial file reads / writes
- Authentication
- More robust error handling
- File metadata support

---

## Related READMEs

- [client/README.md](./client/README.md)
- [server/README.md](./server/README.md)

Enjoy building miniNFS! 🚀
