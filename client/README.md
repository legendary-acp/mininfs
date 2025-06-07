# miniNFS Client

A simple client for the `miniNFS` server — communicates over TCP using a custom protocol:

- Commands supported:

  - `LIST` → list files
  - `READ <filename>` → read file contents
  - `WRITE <filename>` → write file (multi-line input)
  - `APPEND <filename>` → append to file (multi-line input)
  - `QUIT` → close connection

---

## Usage

### Build:

```bash
cargo build --release
```

### Run:

```bash
cargo run -- --server <server_address>
```

If no `--server` is provided, defaults to `127.0.0.1:8080`.

Example:

```bash
cargo run -- --server 192.168.1.100:9090
```

---

## Interactive Commands

Once connected, you can type commands interactively:

```text
> LIST
> READ myfile.txt
> WRITE newfile.txt
Enter file contents (type <EOF> on a new line to finish):
Hello world!
Second line.
<EOF>
> QUIT
```

---

## Command Line Arguments

| Argument           | Description                  | Default          |
| ------------------ | ---------------------------- | ---------------- |
| `--server` or `-s` | Server address to connect to | `127.0.0.1:8080` |

---

## Dependencies

- Rust 1.70+ recommended
- [clap](https://github.com/clap-rs/clap) for CLI argument parsing

---

## Example Architecture

```text
+-----------+         +------------+
| miniNFS   |  <-->   | miniNFS    |
| Client    |  TCP    | Server     |
+-----------+         +------------+
```

---

## Notes

- You can execute multiple commands over a single persistent connection.
- `WRITE` and `APPEND` expect multi-line input terminated by `<EOF>`.
