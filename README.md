# acapy-tools

CLI utility for bulk-managing [ACA-Py](https://github.com/openwallet-foundation/acapy) records via the Admin API.

## Features

Manage three record types — **connections**, **presentation exchanges**, and **credential exchanges** — with these actions:

| Command | Description |
|---------|-------------|
| `connection list` | List connection IDs (up to 5) |
| `connection count` | Count all connections |
| `connection delete` | Delete all connections (and associated OOB invitations) |
| `presex list` | List presentation exchange IDs (up to 5) |
| `presex count` | Count all presentation exchange records |
| `presex delete` | Delete all presentation exchange records |
| `credex list` | List credential exchange IDs (up to 5) |
| `credex count` | Count all credential exchange records |
| `credex delete` | Delete all credential exchange records |

All delete commands support `--dry-run` to preview what would be deleted.

## Installation

### Download pre-built binary (Linux x86_64)

```bash
curl -LO https://github.com/kukgini/acapy-tools/releases/latest/download/acapy-tools-linux-x86_64
chmod +x acapy-tools-linux-x86_64
```

### Build from source

```bash
cargo build --release
```

The binary will be at `target/release/acapy-tools`.

## Usage

```bash
acapy-tools <command> <action> [options]
```

### Options

| Flag | Description |
|------|-------------|
| `-u, --base-url <URL>` | ACA-Py Admin API base URL (default: `http://localhost:8021`) |
| `-t, --token <TOKEN>` | Bearer token for Authorization header |
| `-k, --api-key <KEY>` | API key for X-API-Key header |
| `-b, --batch-size <N>` | Records per batch (default: 5) |
| `-d, --dry-run` | Preview deletions without executing (delete commands only) |

### Examples

```bash
# Count all connections
acapy-tools connection count -u http://localhost:8021

# Delete all connections (dry run)
acapy-tools connection delete --dry-run

# Delete all presentation exchanges with auth
acapy-tools presex delete -t my-bearer-token

# List credential exchange IDs with API key
acapy-tools credex list -k my-api-key
```

## License

[MIT](LICENSE)
