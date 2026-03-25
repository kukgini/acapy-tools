use clap::{Parser, Subcommand};
use futures::future::join_all;
use serde::Deserialize;
use tokio_postgres::NoTls;
use tokio_postgres_rustls::MakeRustlsConnect;

#[derive(Parser, Debug)]
#[command(name = "acapy-tools")]
#[command(about = "ACA-Py utility tools", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Connection management commands
    Connection {
        #[command(subcommand)]
        action: ConnectionCommands,
    },
    /// Presentation exchange management commands
    Presex {
        #[command(subcommand)]
        action: PresexCommands,
    },
    /// Credential exchange management commands
    Credex {
        #[command(subcommand)]
        action: CredexCommands,
    },
    /// Out-of-band invitation management commands
    Oob {
        #[command(subcommand)]
        action: OobCommands,
    },
    /// Direct database management commands
    Db {
        #[command(subcommand)]
        action: DbCommands,
    },
}

#[derive(Subcommand, Debug)]
enum ConnectionCommands {
    /// Delete all connections
    Delete {
        /// Bearer token for Authorization header (optional)
        #[arg(short, long)]
        token: Option<String>,

        /// API key for X-API-Key header (optional)
        #[arg(short = 'k', long)]
        api_key: Option<String>,

        /// ACA-Py Admin API base URL
        #[arg(short = 'u', long, default_value = "http://localhost:8021")]
        base_url: String,

        /// Number of records to fetch per batch
        #[arg(short, long, default_value_t = 5)]
        batch_size: u32,

        /// Show records without deleting
        #[arg(short, long)]
        dry_run: bool,

        /// PostgreSQL connection string for auto VACUUM ANALYZE after each batch
        #[arg(long)]
        db: Option<String>,

        /// Skip TLS for PostgreSQL connection
        #[arg(long)]
        db_no_ssl: bool,
    },
    /// Count all connections using pagination
    Count {
        /// Bearer token for Authorization header (optional)
        #[arg(short, long)]
        token: Option<String>,

        /// API key for X-API-Key header (optional)
        #[arg(short = 'k', long)]
        api_key: Option<String>,

        /// ACA-Py Admin API base URL
        #[arg(short = 'u', long, default_value = "http://localhost:8021")]
        base_url: String,

        /// Number of records to fetch per batch
        #[arg(short, long, default_value_t = 5)]
        batch_size: u32,
    },
    /// List connection IDs (limited to 5)
    List {
        /// Bearer token for Authorization header (optional)
        #[arg(short, long)]
        token: Option<String>,

        /// API key for X-API-Key header (optional)
        #[arg(short = 'k', long)]
        api_key: Option<String>,

        /// ACA-Py Admin API base URL
        #[arg(short = 'u', long, default_value = "http://localhost:8021")]
        base_url: String,
    },
}

#[derive(Subcommand, Debug)]
enum PresexCommands {
    /// Delete all presentation exchange records
    Delete {
        /// Bearer token for Authorization header (optional)
        #[arg(short, long)]
        token: Option<String>,

        /// API key for X-API-Key header (optional)
        #[arg(short = 'k', long)]
        api_key: Option<String>,

        /// ACA-Py Admin API base URL
        #[arg(short = 'u', long, default_value = "http://localhost:8021")]
        base_url: String,

        /// Number of records to fetch per batch
        #[arg(short, long, default_value_t = 5)]
        batch_size: u32,

        /// Show records without deleting
        #[arg(short, long)]
        dry_run: bool,

        /// PostgreSQL connection string for auto VACUUM ANALYZE after each batch
        #[arg(long)]
        db: Option<String>,

        /// Skip TLS for PostgreSQL connection
        #[arg(long)]
        db_no_ssl: bool,
    },
    /// Count all presentation exchange records using pagination
    Count {
        /// Bearer token for Authorization header (optional)
        #[arg(short, long)]
        token: Option<String>,

        /// API key for X-API-Key header (optional)
        #[arg(short = 'k', long)]
        api_key: Option<String>,

        /// ACA-Py Admin API base URL
        #[arg(short = 'u', long, default_value = "http://localhost:8021")]
        base_url: String,

        /// Number of records to fetch per batch
        #[arg(short, long, default_value_t = 5)]
        batch_size: u32,
    },
    /// List presentation exchange IDs (limited to 5)
    List {
        /// Bearer token for Authorization header (optional)
        #[arg(short, long)]
        token: Option<String>,

        /// API key for X-API-Key header (optional)
        #[arg(short = 'k', long)]
        api_key: Option<String>,

        /// ACA-Py Admin API base URL
        #[arg(short = 'u', long, default_value = "http://localhost:8021")]
        base_url: String,
    },
}

#[derive(Subcommand, Debug)]
enum CredexCommands {
    /// Delete all credential exchange records
    Delete {
        /// Bearer token for Authorization header (optional)
        #[arg(short, long)]
        token: Option<String>,

        /// API key for X-API-Key header (optional)
        #[arg(short = 'k', long)]
        api_key: Option<String>,

        /// ACA-Py Admin API base URL
        #[arg(short = 'u', long, default_value = "http://localhost:8021")]
        base_url: String,

        /// Number of records to fetch per batch
        #[arg(short, long, default_value_t = 5)]
        batch_size: u32,

        /// Show records without deleting
        #[arg(short, long)]
        dry_run: bool,

        /// PostgreSQL connection string for auto VACUUM ANALYZE after each batch
        #[arg(long)]
        db: Option<String>,

        /// Skip TLS for PostgreSQL connection
        #[arg(long)]
        db_no_ssl: bool,
    },
    /// Count all credential exchange records using pagination
    Count {
        /// Bearer token for Authorization header (optional)
        #[arg(short, long)]
        token: Option<String>,

        /// API key for X-API-Key header (optional)
        #[arg(short = 'k', long)]
        api_key: Option<String>,

        /// ACA-Py Admin API base URL
        #[arg(short = 'u', long, default_value = "http://localhost:8021")]
        base_url: String,

        /// Number of records to fetch per batch
        #[arg(short, long, default_value_t = 5)]
        batch_size: u32,
    },
    /// List credential exchange IDs (limited to 5)
    List {
        /// Bearer token for Authorization header (optional)
        #[arg(short, long)]
        token: Option<String>,

        /// API key for X-API-Key header (optional)
        #[arg(short = 'k', long)]
        api_key: Option<String>,

        /// ACA-Py Admin API base URL
        #[arg(short = 'u', long, default_value = "http://localhost:8021")]
        base_url: String,
    },
}

#[derive(Subcommand, Debug)]
enum OobCommands {
    /// Delete all out-of-band invitations
    Delete {
        /// Bearer token for Authorization header (optional)
        #[arg(short, long)]
        token: Option<String>,

        /// API key for X-API-Key header (optional)
        #[arg(short = 'k', long)]
        api_key: Option<String>,

        /// ACA-Py Admin API base URL
        #[arg(short = 'u', long, default_value = "http://localhost:8021")]
        base_url: String,

        /// Number of records to fetch per batch
        #[arg(short, long, default_value_t = 5)]
        batch_size: u32,

        /// Show records without deleting
        #[arg(short, long)]
        dry_run: bool,

        /// PostgreSQL connection string for auto VACUUM ANALYZE after each batch
        #[arg(long)]
        db: Option<String>,

        /// Skip TLS for PostgreSQL connection
        #[arg(long)]
        db_no_ssl: bool,
    },
    /// Count all out-of-band invitations using pagination
    Count {
        /// Bearer token for Authorization header (optional)
        #[arg(short, long)]
        token: Option<String>,

        /// API key for X-API-Key header (optional)
        #[arg(short = 'k', long)]
        api_key: Option<String>,

        /// ACA-Py Admin API base URL
        #[arg(short = 'u', long, default_value = "http://localhost:8021")]
        base_url: String,

        /// Number of records to fetch per batch
        #[arg(short, long, default_value_t = 5)]
        batch_size: u32,
    },
    /// List out-of-band invitation IDs (limited to 5)
    List {
        /// Bearer token for Authorization header (optional)
        #[arg(short, long)]
        token: Option<String>,

        /// API key for X-API-Key header (optional)
        #[arg(short = 'k', long)]
        api_key: Option<String>,

        /// ACA-Py Admin API base URL
        #[arg(short = 'u', long, default_value = "http://localhost:8021")]
        base_url: String,
    },
}

#[derive(Subcommand, Debug)]
enum DbCommands {
    /// Run VACUUM ANALYZE on the database
    Vacuum {
        /// PostgreSQL connection string (e.g., "host=localhost user=postgres dbname=acapy")
        #[arg(short, long)]
        connection: String,

        /// Skip TLS for PostgreSQL connection
        #[arg(long)]
        no_ssl: bool,
    },
}

#[derive(Debug, Deserialize)]
struct Connection {
    connection_id: String,
    state: Option<String>,
    their_label: Option<String>,
    invitation_msg_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ConnectionsResponse {
    results: Vec<Connection>,
}

#[derive(Debug, Deserialize)]
struct PresentationExchange {
    presentation_exchange_id: String,
    state: Option<String>,
    connection_id: Option<String>,
    #[serde(default)]
    parent_thread_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PresentationExchangeResponse {
    results: Vec<PresentationExchange>,
}

#[derive(Debug, Deserialize)]
struct CredentialExchange {
    credential_exchange_id: String,
    #[serde(default)]
    parent_thread_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CredentialExchangeResponse {
    results: Vec<CredentialExchange>,
}

#[derive(Debug, Deserialize)]
struct OobInvitation {
    oob_id: String,
    invi_msg_id: Option<String>,
    state: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OobInvitationResponse {
    results: Vec<OobInvitation>,
}

fn build_client(token: Option<&str>, api_key: Option<&str>) -> Result<reqwest::Client, String> {
    let mut headers = reqwest::header::HeaderMap::new();
    if let Some(t) = token {
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", t)
                .parse()
                .map_err(|e: reqwest::header::InvalidHeaderValue| {
                    format!("Invalid token value: {}", e)
                })?,
        );
    }
    if let Some(key) = api_key {
        headers.insert(
            reqwest::header::HeaderName::from_static("x-api-key"),
            key.parse()
                .map_err(|e: reqwest::header::InvalidHeaderValue| {
                    format!("Invalid api-key value: {}", e)
                })?,
        );
    }
    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))
}

async fn get_connections(
    client: &reqwest::Client,
    base_url: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<Connection>, reqwest::Error> {
    let url = format!("{}/connections?limit={}&offset={}", base_url, limit, offset);
    let resp = client.get(&url).send().await?.error_for_status()?;
    let data: ConnectionsResponse = resp.json().await?;
    Ok(data.results)
}

async fn delete_connection(
    client: &reqwest::Client,
    base_url: &str,
    conn_id: &str,
) -> Result<(), reqwest::Error> {
    let url = format!("{}/connections/{}", base_url, conn_id);
    client.delete(&url).send().await?.error_for_status()?;
    Ok(())
}

async fn get_presentation_exchanges(
    client: &reqwest::Client,
    base_url: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<PresentationExchange>, reqwest::Error> {
    let url = format!("{}/present-proof/records?limit={}&offset={}", base_url, limit, offset);
    let resp = client.get(&url).send().await?.error_for_status()?;
    let data: PresentationExchangeResponse = resp.json().await?;
    Ok(data.results)
}

async fn delete_presentation_exchange(
    client: &reqwest::Client,
    base_url: &str,
    pres_ex_id: &str,
) -> Result<(), reqwest::Error> {
    let url = format!("{}/present-proof/records/{}", base_url, pres_ex_id);
    client.delete(&url).send().await?.error_for_status()?;
    Ok(())
}

async fn get_credential_exchanges(
    client: &reqwest::Client,
    base_url: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<CredentialExchange>, reqwest::Error> {
    let url = format!(
        "{}/issue-credential/records?limit={}&offset={}",
        base_url, limit, offset
    );
    let resp = client.get(&url).send().await?.error_for_status()?;
    let data: CredentialExchangeResponse = resp.json().await?;
    Ok(data.results)
}

async fn delete_credential_exchange(
    client: &reqwest::Client,
    base_url: &str,
    cred_ex_id: &str,
) -> Result<(), reqwest::Error> {
    let url = format!("{}/issue-credential/records/{}", base_url, cred_ex_id);
    client.delete(&url).send().await?.error_for_status()?;
    Ok(())
}

async fn get_oob_invitations(
    client: &reqwest::Client,
    base_url: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<OobInvitation>, reqwest::Error> {
    let url = format!(
        "{}/out-of-band/invitations?limit={}&offset={}",
        base_url, limit, offset
    );
    let resp = client.get(&url).send().await?.error_for_status()?;
    let data: OobInvitationResponse = resp.json().await?;
    Ok(data.results)
}

async fn delete_oob_invitation(
    client: &reqwest::Client,
    base_url: &str,
    oob_id: &str,
) -> Result<(), reqwest::Error> {
    let url = format!("{}/out-of-band/invitations/{}", base_url, oob_id);
    client.delete(&url).send().await?.error_for_status()?;
    Ok(())
}

#[tokio::main]
async fn main() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install default CryptoProvider");

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Connection { action } => match action {
            ConnectionCommands::Delete {
                token,
                api_key,
                base_url,
                batch_size,
                dry_run,
                db,
                db_no_ssl,
            } => {
                let db_client = connect_db(db.as_deref(), db_no_ssl).await;
                delete_all_connections(token.as_deref(), api_key.as_deref(), &base_url, batch_size, dry_run, db_client.as_ref())
                    .await
            }
            ConnectionCommands::Count {
                token,
                api_key,
                base_url,
                batch_size,
            } => {
                count_connections(token.as_deref(), api_key.as_deref(), &base_url, batch_size).await
            }
            ConnectionCommands::List {
                token,
                api_key,
                base_url,
            } => {
                list_connections(token.as_deref(), api_key.as_deref(), &base_url).await
            }
        },
        Commands::Presex { action } => match action {
            PresexCommands::Delete {
                token,
                api_key,
                base_url,
                batch_size,
                dry_run,
                db,
                db_no_ssl,
            } => {
                let db_client = connect_db(db.as_deref(), db_no_ssl).await;
                delete_all_presex(token.as_deref(), api_key.as_deref(), &base_url, batch_size, dry_run, db_client.as_ref())
                    .await
            }
            PresexCommands::Count {
                token,
                api_key,
                base_url,
                batch_size,
            } => {
                count_presex(token.as_deref(), api_key.as_deref(), &base_url, batch_size).await
            }
            PresexCommands::List {
                token,
                api_key,
                base_url,
            } => {
                list_presex(token.as_deref(), api_key.as_deref(), &base_url).await
            }
        },
        Commands::Credex { action } => match action {
            CredexCommands::Delete {
                token,
                api_key,
                base_url,
                batch_size,
                dry_run,
                db,
                db_no_ssl,
            } => {
                let db_client = connect_db(db.as_deref(), db_no_ssl).await;
                delete_all_credex(token.as_deref(), api_key.as_deref(), &base_url, batch_size, dry_run, db_client.as_ref())
                    .await
            }
            CredexCommands::Count {
                token,
                api_key,
                base_url,
                batch_size,
            } => {
                count_credex(token.as_deref(), api_key.as_deref(), &base_url, batch_size).await
            }
            CredexCommands::List {
                token,
                api_key,
                base_url,
            } => {
                list_credex(token.as_deref(), api_key.as_deref(), &base_url).await
            }
        },
        Commands::Oob { action } => match action {
            OobCommands::Delete {
                token,
                api_key,
                base_url,
                batch_size,
                dry_run,
                db,
                db_no_ssl,
            } => {
                let db_client = connect_db(db.as_deref(), db_no_ssl).await;
                delete_all_oob(token.as_deref(), api_key.as_deref(), &base_url, batch_size, dry_run, db_client.as_ref())
                    .await
            }
            OobCommands::Count {
                token,
                api_key,
                base_url,
                batch_size,
            } => {
                count_oob(token.as_deref(), api_key.as_deref(), &base_url, batch_size).await
            }
            OobCommands::List {
                token,
                api_key,
                base_url,
            } => {
                list_oob(token.as_deref(), api_key.as_deref(), &base_url).await
            }
        },
        Commands::Db { action } => match action {
            DbCommands::Vacuum { connection, no_ssl } => {
                vacuum_analyze(&connection, no_ssl).await
            }
        },
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn delete_all_connections(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
    batch_size: u32,
    dry_run: bool,
    db_client: Option<&tokio_postgres::Client>,
) -> Result<(), String> {
    let client = build_client(token, api_key)?;
    let mut total_deleted = 0u32;
    let mut since_last_vacuum = 0u32;
    let mut had_error = false;

    loop {
        let connections = match get_connections(&client, base_url, batch_size, 0).await {
            Ok(conns) => conns,
            Err(e) => {
                eprintln!("Failed to fetch connections: {}", e);
                had_error = true;
                break;
            }
        };

        if connections.is_empty() {
            println!("No more connections found.");
            break;
        }

        println!("Found {} connection(s)", connections.len());

        if dry_run {
            for conn in &connections {
                let state = conn.state.as_deref().unwrap_or("unknown");
                let label = conn.their_label.as_deref().unwrap_or("N/A");
                let oob = conn.invitation_msg_id.as_deref().unwrap_or("N/A");
                println!(
                    "  [DRY-RUN] Would delete: {} (state={}, label={}, oob={})",
                    conn.connection_id, state, label, oob
                );
            }
            break;
        } else {
            // Delete OOB invitations in parallel
            let oob_futures: Vec<_> = connections.iter()
                .filter_map(|conn| conn.invitation_msg_id.as_deref().map(|oob_id| {
                    let client = &client;
                    let oob_id = oob_id.to_string();
                    async move {
                        let _ = delete_oob_invitation(client, base_url, &oob_id).await;
                    }
                }))
                .collect();
            join_all(oob_futures).await;

            // Delete connections in parallel
            let delete_futures: Vec<_> = connections.iter().map(|conn| {
                let client = &client;
                async move {
                    match delete_connection(client, base_url, &conn.connection_id).await {
                        Ok(()) => {
                            let state = conn.state.as_deref().unwrap_or("unknown");
                            let label = conn.their_label.as_deref().unwrap_or("N/A");
                            println!(
                                "  Deleted: {} (state={}, label={})",
                                conn.connection_id, state, label
                            );
                            true
                        }
                        Err(e) => {
                            eprintln!("  Failed to delete {}: {}", conn.connection_id, e);
                            false
                        }
                    }
                }
            }).collect();
            let results = join_all(delete_futures).await;
            let batch_deleted = results.iter().filter(|&&ok| ok).count() as u32;
            total_deleted += batch_deleted;
            if batch_deleted == 0 {
                eprintln!("Warning: no records deleted in this batch. Stopping.");
                break;
            }
            since_last_vacuum += batch_deleted;
            if since_last_vacuum >= 1000 {
                run_vacuum(db_client).await;
                since_last_vacuum = 0;
            }
        }
    }

    if dry_run {
        println!("Dry run complete. No connections were deleted.");
    } else {
        run_vacuum(db_client).await;
        println!("Done. Total deleted: {}", total_deleted);
    }

    if had_error { Err("Failed to complete connection deletion".to_string()) } else { Ok(()) }
}

async fn count_connections(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
    batch_size: u32,
) -> Result<(), String> {
    let client = build_client(token, api_key)?;
    let mut total_count = 0u32;
    let mut offset = 0u32;

    loop {
        let records = match get_connections(&client, base_url, batch_size, offset).await {
            Ok(recs) => recs,
            Err(e) => {
                return Err(format!("Failed to fetch connections: {}", e));
            }
        };

        let batch_count = records.len() as u32;
        if batch_count == 0 {
            break;
        }

        total_count += batch_count;
        offset += batch_count;

        println!("Fetched {} records (total so far: {})", batch_count, total_count);

        if batch_count < batch_size {
            break;
        }
    }

    println!("Total connection records: {}", total_count);
    Ok(())
}

async fn delete_all_presex(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
    batch_size: u32,
    dry_run: bool,
    db_client: Option<&tokio_postgres::Client>,
) -> Result<(), String> {
    let client = build_client(token, api_key)?;
    let mut total_deleted = 0u32;
    let mut since_last_vacuum = 0u32;
    let mut had_error = false;

    loop {
        let records = match get_presentation_exchanges(&client, base_url, batch_size, 0).await {
            Ok(recs) => recs,
            Err(e) => {
                eprintln!("Failed to fetch presentation exchanges: {}", e);
                had_error = true;
                break;
            }
        };

        if records.is_empty() {
            println!("No more presentation exchange records found.");
            break;
        }

        println!("Found {} presentation exchange record(s)", records.len());

        if dry_run {
            for rec in &records {
                let state = rec.state.as_deref().unwrap_or("unknown");
                let conn_id = rec.connection_id.as_deref().unwrap_or("N/A");
                let oob = rec.parent_thread_id.as_deref().unwrap_or("N/A");
                println!(
                    "  [DRY-RUN] Would delete: {} (state={}, connection_id={}, oob={})",
                    rec.presentation_exchange_id, state, conn_id, oob
                );
            }
            break;
        } else {
            // Delete OOB invitations in parallel
            let oob_futures: Vec<_> = records.iter()
                .filter_map(|rec| rec.parent_thread_id.as_deref().map(|oob_id| {
                    let client = &client;
                    let oob_id = oob_id.to_string();
                    async move {
                        let _ = delete_oob_invitation(client, base_url, &oob_id).await;
                    }
                }))
                .collect();
            join_all(oob_futures).await;

            // Delete presentation exchanges in parallel
            let delete_futures: Vec<_> = records.iter().map(|rec| {
                let client = &client;
                async move {
                    match delete_presentation_exchange(client, base_url, &rec.presentation_exchange_id).await {
                        Ok(()) => {
                            let state = rec.state.as_deref().unwrap_or("unknown");
                            let conn_id = rec.connection_id.as_deref().unwrap_or("N/A");
                            println!(
                                "  Deleted: {} (state={}, connection_id={})",
                                rec.presentation_exchange_id, state, conn_id
                            );
                            true
                        }
                        Err(e) => {
                            eprintln!("  Failed to delete {}: {}", rec.presentation_exchange_id, e);
                            false
                        }
                    }
                }
            }).collect();
            let results = join_all(delete_futures).await;
            let batch_deleted = results.iter().filter(|&&ok| ok).count() as u32;
            total_deleted += batch_deleted;
            if batch_deleted == 0 {
                eprintln!("Warning: no records deleted in this batch. Stopping.");
                break;
            }
            since_last_vacuum += batch_deleted;
            if since_last_vacuum >= 1000 {
                run_vacuum(db_client).await;
                since_last_vacuum = 0;
            }
        }
    }

    if dry_run {
        println!("Dry run complete. No presentation exchanges were deleted.");
    } else {
        run_vacuum(db_client).await;
        println!("Done. Total deleted: {}", total_deleted);
    }

    if had_error { Err("Failed to complete presex deletion".to_string()) } else { Ok(()) }
}

async fn count_presex(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
    batch_size: u32,
) -> Result<(), String> {
    let client = build_client(token, api_key)?;
    let mut total_count = 0u32;
    let mut offset = 0u32;

    loop {
        let records = match get_presentation_exchanges(&client, base_url, batch_size, offset).await {
            Ok(recs) => recs,
            Err(e) => {
                return Err(format!("Failed to fetch presentation exchanges: {}", e));
            }
        };

        let batch_count = records.len() as u32;
        if batch_count == 0 {
            break;
        }

        total_count += batch_count;
        offset += batch_count;

        println!("Fetched {} records (total so far: {})", batch_count, total_count);

        if batch_count < batch_size {
            break;
        }
    }

    println!("Total presentation exchange records: {}", total_count);
    Ok(())
}

async fn count_credex(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
    batch_size: u32,
) -> Result<(), String> {
    let client = build_client(token, api_key)?;
    let mut total_count = 0u32;
    let mut offset = 0u32;

    loop {
        let records = match get_credential_exchanges(&client, base_url, batch_size, offset).await {
            Ok(recs) => recs,
            Err(e) => {
                return Err(format!("Failed to fetch credential exchanges: {}", e));
            }
        };

        let batch_count = records.len() as u32;
        if batch_count == 0 {
            break;
        }

        total_count += batch_count;
        offset += batch_count;

        println!("Fetched {} records (total so far: {})", batch_count, total_count);

        if batch_count < batch_size {
            break;
        }
    }

    println!("Total credential exchange records: {}", total_count);
    Ok(())
}

async fn delete_all_credex(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
    batch_size: u32,
    dry_run: bool,
    db_client: Option<&tokio_postgres::Client>,
) -> Result<(), String> {
    let client = build_client(token, api_key)?;
    let mut total_deleted = 0u32;
    let mut since_last_vacuum = 0u32;
    let mut had_error = false;

    loop {
        let records = match get_credential_exchanges(&client, base_url, batch_size, 0).await {
            Ok(recs) => recs,
            Err(e) => {
                eprintln!("Failed to fetch credential exchanges: {}", e);
                had_error = true;
                break;
            }
        };

        if records.is_empty() {
            println!("No more credential exchange records found.");
            break;
        }

        println!("Found {} credential exchange record(s)", records.len());

        if dry_run {
            for rec in &records {
                let oob = rec.parent_thread_id.as_deref().unwrap_or("N/A");
                println!(
                    "  [DRY-RUN] Would delete: {} (oob={})",
                    rec.credential_exchange_id, oob
                );
            }
            break;
        } else {
            // Delete OOB invitations in parallel
            let oob_futures: Vec<_> = records.iter()
                .filter_map(|rec| rec.parent_thread_id.as_deref().map(|oob_id| {
                    let client = &client;
                    let oob_id = oob_id.to_string();
                    async move {
                        let _ = delete_oob_invitation(client, base_url, &oob_id).await;
                    }
                }))
                .collect();
            join_all(oob_futures).await;

            // Delete credential exchanges in parallel
            let delete_futures: Vec<_> = records.iter().map(|rec| {
                let client = &client;
                async move {
                    match delete_credential_exchange(client, base_url, &rec.credential_exchange_id).await {
                        Ok(()) => {
                            println!("  Deleted: {}", rec.credential_exchange_id);
                            true
                        }
                        Err(e) => {
                            eprintln!("  Failed to delete {}: {}", rec.credential_exchange_id, e);
                            false
                        }
                    }
                }
            }).collect();
            let results = join_all(delete_futures).await;
            let batch_deleted = results.iter().filter(|&&ok| ok).count() as u32;
            total_deleted += batch_deleted;
            if batch_deleted == 0 {
                eprintln!("Warning: no records deleted in this batch. Stopping.");
                break;
            }
            since_last_vacuum += batch_deleted;
            if since_last_vacuum >= 1000 {
                run_vacuum(db_client).await;
                since_last_vacuum = 0;
            }
        }
    }

    if dry_run {
        println!("Dry run complete. No credential exchanges were deleted.");
    } else {
        run_vacuum(db_client).await;
        println!("Done. Total deleted: {}", total_deleted);
    }

    if had_error { Err("Failed to complete credex deletion".to_string()) } else { Ok(()) }
}

async fn list_connections(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
) -> Result<(), String> {
    let client = build_client(token, api_key)?;
    let limit = 5;

    let connections = match get_connections(&client, base_url, limit, 0).await {
        Ok(conns) => conns,
        Err(e) => {
            return Err(format!("Failed to fetch connections: {}", e));
        }
    };

    for conn in connections {
        println!("{}", conn.connection_id);
    }
    Ok(())
}

async fn list_presex(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
) -> Result<(), String> {
    let client = build_client(token, api_key)?;
    let limit = 5;

    let records = match get_presentation_exchanges(&client, base_url, limit, 0).await {
        Ok(recs) => recs,
        Err(e) => {
            return Err(format!("Failed to fetch presentation exchanges: {}", e));
        }
    };

    for rec in records {
        println!("{}", rec.presentation_exchange_id);
    }
    Ok(())
}

async fn list_credex(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
) -> Result<(), String> {
    let client = build_client(token, api_key)?;
    let limit = 5;

    let records = match get_credential_exchanges(&client, base_url, limit, 0).await {
        Ok(recs) => recs,
        Err(e) => {
            return Err(format!("Failed to fetch credential exchanges: {}", e));
        }
    };

    for rec in records {
        println!("{}", rec.credential_exchange_id);
    }
    Ok(())
}

async fn delete_all_oob(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
    batch_size: u32,
    dry_run: bool,
    db_client: Option<&tokio_postgres::Client>,
) -> Result<(), String> {
    let client = build_client(token, api_key)?;
    let mut total_deleted = 0u32;
    let mut since_last_vacuum = 0u32;
    let mut had_error = false;

    loop {
        let records = match get_oob_invitations(&client, base_url, batch_size, 0).await {
            Ok(recs) => recs,
            Err(e) => {
                eprintln!("Failed to fetch OOB invitations: {}", e);
                had_error = true;
                break;
            }
        };

        if records.is_empty() {
            println!("No more OOB invitation records found.");
            break;
        }

        println!("Found {} OOB invitation record(s)", records.len());

        if dry_run {
            for rec in &records {
                let state = rec.state.as_deref().unwrap_or("unknown");
                let invi_msg_id = rec.invi_msg_id.as_deref().unwrap_or("N/A");
                println!(
                    "  [DRY-RUN] Would delete: {} (state={}, invi_msg_id={})",
                    rec.oob_id, state, invi_msg_id
                );
            }
            break;
        } else {
            // Delete OOB invitations in parallel
            let delete_futures: Vec<_> = records.iter().map(|rec| {
                let client = &client;
                async move {
                    match delete_oob_invitation(client, base_url, &rec.oob_id).await {
                        Ok(()) => {
                            let state = rec.state.as_deref().unwrap_or("unknown");
                            let invi_msg_id = rec.invi_msg_id.as_deref().unwrap_or("N/A");
                            println!(
                                "  Deleted: {} (state={}, invi_msg_id={})",
                                rec.oob_id, state, invi_msg_id
                            );
                            true
                        }
                        Err(e) => {
                            eprintln!("  Failed to delete {}: {}", rec.oob_id, e);
                            false
                        }
                    }
                }
            }).collect();
            let results = join_all(delete_futures).await;
            let batch_deleted = results.iter().filter(|&&ok| ok).count() as u32;
            total_deleted += batch_deleted;
            if batch_deleted == 0 {
                eprintln!("Warning: no records deleted in this batch. Stopping.");
                break;
            }
            since_last_vacuum += batch_deleted;
            if since_last_vacuum >= 1000 {
                run_vacuum(db_client).await;
                since_last_vacuum = 0;
            }
        }
    }

    if dry_run {
        println!("Dry run complete. No OOB invitations were deleted.");
    } else {
        run_vacuum(db_client).await;
        println!("Done. Total deleted: {}", total_deleted);
    }

    if had_error { Err("Failed to complete OOB deletion".to_string()) } else { Ok(()) }
}

async fn count_oob(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
    batch_size: u32,
) -> Result<(), String> {
    let client = build_client(token, api_key)?;
    let mut total_count = 0u32;
    let mut offset = 0u32;

    loop {
        let records = match get_oob_invitations(&client, base_url, batch_size, offset).await {
            Ok(recs) => recs,
            Err(e) => {
                return Err(format!("Failed to fetch OOB invitations: {}", e));
            }
        };

        let batch_count = records.len() as u32;
        if batch_count == 0 {
            break;
        }

        total_count += batch_count;
        offset += batch_count;

        println!("Fetched {} records (total so far: {})", batch_count, total_count);

        if batch_count < batch_size {
            break;
        }
    }

    println!("Total OOB invitation records: {}", total_count);
    Ok(())
}

async fn list_oob(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
) -> Result<(), String> {
    let client = build_client(token, api_key)?;
    let limit = 5;

    let records = match get_oob_invitations(&client, base_url, limit, 0).await {
        Ok(recs) => recs,
        Err(e) => {
            return Err(format!("Failed to fetch OOB invitations: {}", e));
        }
    };

    for rec in records {
        println!("{}", rec.oob_id);
    }
    Ok(())
}

async fn connect_db(connection: Option<&str>, no_ssl: bool) -> Option<tokio_postgres::Client> {
    let conn_str = connection?;

    if no_ssl {
        println!("Connecting to PostgreSQL...");
        match tokio_postgres::connect(conn_str, NoTls).await {
            Ok((client, connection_task)) => {
                tokio::spawn(async move {
                    if let Err(e) = connection_task.await {
                        eprintln!("Database connection error: {}", e);
                    }
                });
                println!("Connected to PostgreSQL.");
                Some(client)
            }
            Err(e) => {
                eprintln!("Failed to connect to database: {}", e);
                None
            }
        }
    } else {
        println!("Connecting to PostgreSQL (SSL)...");
        let mut root_store = rustls::RootCertStore::empty();
        root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
        let tls_config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        let tls = MakeRustlsConnect::new(tls_config);
        match tokio_postgres::connect(conn_str, tls).await {
            Ok((client, connection_task)) => {
                tokio::spawn(async move {
                    if let Err(e) = connection_task.await {
                        eprintln!("Database connection error: {}", e);
                    }
                });
                println!("Connected to PostgreSQL (SSL).");
                Some(client)
            }
            Err(e) => {
                eprintln!("Failed to connect to database: {}", e);
                None
            }
        }
    }
}

async fn run_vacuum(db_client: Option<&tokio_postgres::Client>) {
    if let Some(client) = db_client {
        println!("Running VACUUM ANALYZE...");
        match client.batch_execute("VACUUM ANALYZE").await {
            Ok(()) => println!("VACUUM ANALYZE completed."),
            Err(e) => eprintln!("Failed to run VACUUM ANALYZE: {}", e),
        }
    }
}

async fn vacuum_analyze(connection: &str, no_ssl: bool) -> Result<(), String> {
    let db_client = connect_db(Some(connection), no_ssl).await;
    if db_client.is_none() {
        return Err("Failed to connect to database".to_string());
    }
    run_vacuum(db_client.as_ref()).await;
    Ok(())
}
