# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build                # dev build
cargo build --release      # release build
cargo check                # type-check without building
```

CI builds a static musl binary for Linux:
```bash
cargo build --release --target x86_64-unknown-linux-musl
```

There are no tests or linting configured.

## Architecture

Single-binary CLI tool in Rust. Uses clap derive for CLI parsing.

### Module structure

- `src/main.rs` — Entry point: rustls init, CLI parse, dispatch to `api::` or `db::`.
- `src/cli.rs` — All clap structs. Shared `ApiArgs`/`BatchArgs`/`DeleteArgs` are flattened into each subcommand via `#[command(flatten)]`.
- `src/models.rs` — Record types (`Connection`, `PresentationExchange`, `CredentialExchange`, `OobInvitation`), generic `PagedResponse<T>`, and the `Record` trait.
- `src/api.rs` — REST API client: `ResourceConfig` (URL paths per resource type), generic `run_delete<T>`/`run_count<T>`/`run_list<T>` orchestration. No `aries-askar` imports.
- `src/db.rs` — Direct DB operations via `aries-askar`: `WalletConfig`, `open_store`, `KNOWN_CATEGORIES`, `db_list_profiles`/`db_list_categories`/`db_count`/`db_delete`. No `reqwest` imports.

### Two modes of operation

1. **REST API mode** (`connection`, `presex`, `credex`, `oob` commands) — manages ACA-Py records via the Admin API using `reqwest`. Each record type has `list`, `count`, and `delete` subcommands. Delete commands fetch records in batches (offset=0 each loop since records shift after deletion) and delete them in parallel using `futures::join_all`.

2. **Direct DB mode** (`db` commands) — operates directly on the Askar wallet store using the `aries-askar` crate. Reads connection info from ACA-Py environment variables (`ACAPY_WALLET_STORAGE_CONFIG`, `ACAPY_WALLET_STORAGE_CREDS`, `ACAPY_WALLET_NAME`, `ACAPY_WALLET_KEY`, `ACAPY_WALLET_KEY_DERIVATION_METHOD`). Supports multitenancy managed mode with profile-based tenant isolation. Subcommands: `list-profiles`, `list-categories`, `count`, `delete`.

### Key patterns

- All REST API record types share generic orchestration via the `Record` trait and `ResourceConfig`. To add a new record type: define the struct in `models.rs`, implement `Record`, add a `ResourceConfig` constant in `api.rs`, and add CLI subcommands in `cli.rs`.
- The `Record` trait provides `oob_id()` for side-deletion of associated OOB invitations. `Connection` returns `invitation_msg_id`; `PresentationExchange` and `CredentialExchange` return `parent_thread_id`; `OobInvitation` returns `None`.
- DB credentials in URLs are percent-encoded via a local `percent_encode()` function in `db.rs`.
- `rustls::crypto::ring::default_provider().install_default()` is called at main() startup — required by both reqwest (rustls-tls) and aries-askar (sqlx-postgres with rustls).
