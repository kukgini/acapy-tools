use aries_askar::{Store, StoreKeyMethod};
use aries_askar::storage::{Argon2Level, KdfMethod};
use clap::{Parser, Subcommand};
use futures::future::join_all;
use serde::Deserialize;

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

        /// Print each deletion result
        #[arg(short, long)]
        verbose: bool,
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

        /// Print each deletion result
        #[arg(short, long)]
        verbose: bool,
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

        /// Print each deletion result
        #[arg(short, long)]
        verbose: bool,
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

        /// Print each deletion result
        #[arg(short, long)]
        verbose: bool,
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
    /// List all wallet profiles (tenants)
    ListProfiles {
        /// Override wallet database name (for DatabasePerWallet tenant access)
        #[arg(long)]
        wallet_name: Option<String>,
    },

    /// List unique record categories in a profile
    ListCategories {
        /// Override wallet database name (for DatabasePerWallet tenant access)
        #[arg(long)]
        wallet_name: Option<String>,

        /// Wallet profile name (for MultiWalletSingleTable)
        #[arg(long)]
        profile: Option<String>,
    },

    /// Count records by category in a profile
    Count {
        /// Override wallet database name (for DatabasePerWallet tenant access)
        #[arg(long)]
        wallet_name: Option<String>,

        /// Wallet profile name (for MultiWalletSingleTable)
        #[arg(long)]
        profile: Option<String>,

        /// Record category (e.g., "connection", "oob_record")
        #[arg(long)]
        category: String,
    },

    /// Delete all records of a category in a profile
    Delete {
        /// Override wallet database name (for DatabasePerWallet tenant access)
        #[arg(long)]
        wallet_name: Option<String>,

        /// Wallet profile name (for MultiWalletSingleTable)
        #[arg(long)]
        profile: Option<String>,

        /// Record category (e.g., "connection", "oob_record")
        #[arg(long)]
        category: String,

        /// Show what would be deleted without actually deleting
        #[arg(short, long)]
        dry_run: bool,
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
        "{}/out-of-band/records?limit={}&offset={}",
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
                verbose,
            } => {
                delete_all_connections(token.as_deref(), api_key.as_deref(), &base_url, batch_size, dry_run, verbose)
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
                verbose,
            } => {
                delete_all_presex(token.as_deref(), api_key.as_deref(), &base_url, batch_size, dry_run, verbose)
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
                verbose,
            } => {
                delete_all_credex(token.as_deref(), api_key.as_deref(), &base_url, batch_size, dry_run, verbose)
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
                verbose,
            } => {
                delete_all_oob(token.as_deref(), api_key.as_deref(), &base_url, batch_size, dry_run, verbose)
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
            DbCommands::ListProfiles { wallet_name } => {
                db_list_profiles(wallet_name.as_deref()).await
            }
            DbCommands::ListCategories { wallet_name, profile } => {
                db_list_categories(wallet_name.as_deref(), profile.as_deref()).await
            }
            DbCommands::Count { wallet_name, profile, category } => {
                db_count(wallet_name.as_deref(), profile.as_deref(), &category).await
            }
            DbCommands::Delete { wallet_name, profile, category, dry_run } => {
                db_delete(wallet_name.as_deref(), profile.as_deref(), &category, dry_run).await
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
    verbose: bool,
) -> Result<(), String> {
    let client = build_client(token, api_key)?;
    let mut total_deleted = 0u32;
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
                            if verbose {
                                let state = conn.state.as_deref().unwrap_or("unknown");
                                let label = conn.their_label.as_deref().unwrap_or("N/A");
                                println!(
                                    "  Deleted: {} (state={}, label={})",
                                    conn.connection_id, state, label
                                );
                            }
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
        }
    }

    if dry_run {
        println!("Dry run complete. No connections were deleted.");
    } else {
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
    verbose: bool,
) -> Result<(), String> {
    let client = build_client(token, api_key)?;
    let mut total_deleted = 0u32;
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
                            if verbose {
                                let state = rec.state.as_deref().unwrap_or("unknown");
                                let conn_id = rec.connection_id.as_deref().unwrap_or("N/A");
                                println!(
                                    "  Deleted: {} (state={}, connection_id={})",
                                    rec.presentation_exchange_id, state, conn_id
                                );
                            }
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
        }
    }

    if dry_run {
        println!("Dry run complete. No presentation exchanges were deleted.");
    } else {
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
    verbose: bool,
) -> Result<(), String> {
    let client = build_client(token, api_key)?;
    let mut total_deleted = 0u32;
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
                            if verbose {
                                println!("  Deleted: {}", rec.credential_exchange_id);
                            }
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
        }
    }

    if dry_run {
        println!("Dry run complete. No credential exchanges were deleted.");
    } else {
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
    verbose: bool,
) -> Result<(), String> {
    let client = build_client(token, api_key)?;
    let mut total_deleted = 0u32;
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
                            if verbose {
                                let state = rec.state.as_deref().unwrap_or("unknown");
                                let invi_msg_id = rec.invi_msg_id.as_deref().unwrap_or("N/A");
                                println!(
                                    "  Deleted: {} (state={}, invi_msg_id={})",
                                    rec.oob_id, state, invi_msg_id
                                );
                            }
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
        }
    }

    if dry_run {
        println!("Dry run complete. No OOB invitations were deleted.");
    } else {
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

async fn open_store_from_env(wallet_name_override: Option<&str>, is_tenant: bool) -> Result<Store, String> {
    let storage_config = std::env::var("ACAPY_WALLET_STORAGE_CONFIG")
        .map_err(|_| "ACAPY_WALLET_STORAGE_CONFIG is not set".to_string())?;
    let storage_creds = std::env::var("ACAPY_WALLET_STORAGE_CREDS")
        .map_err(|_| "ACAPY_WALLET_STORAGE_CREDS is not set".to_string())?;
    let wallet_name = if let Some(name) = wallet_name_override {
        name.to_string()
    } else if is_tenant {
        let mt_config = std::env::var("ACAPY_MULTITENANCY_CONFIGURATION")
            .map_err(|_| "ACAPY_MULTITENANCY_CONFIGURATION is not set".to_string())?;
        let mt: serde_json::Value = serde_json::from_str(&mt_config)
            .map_err(|e| format!("Failed to parse ACAPY_MULTITENANCY_CONFIGURATION: {}", e))?;
        mt["wallet_name"]
            .as_str()
            .ok_or("ACAPY_MULTITENANCY_CONFIGURATION missing 'wallet_name' field")?
            .to_string()
    } else {
        std::env::var("ACAPY_WALLET_NAME")
            .map_err(|_| "ACAPY_WALLET_NAME is not set".to_string())?
    };
    let wallet_key = std::env::var("ACAPY_WALLET_KEY")
        .map_err(|_| "ACAPY_WALLET_KEY is not set".to_string())?;
    let key_derivation = std::env::var("ACAPY_WALLET_KEY_DERIVATION_METHOD")
        .unwrap_or_else(|_| "ARGON2I_MOD".to_string());

    let config: serde_json::Value = serde_json::from_str(&storage_config)
        .map_err(|e| format!("Failed to parse ACAPY_WALLET_STORAGE_CONFIG: {}", e))?;
    let db_url = config["url"]
        .as_str()
        .ok_or("ACAPY_WALLET_STORAGE_CONFIG missing 'url' field")?;

    let creds: serde_json::Value = serde_json::from_str(&storage_creds)
        .map_err(|e| format!("Failed to parse ACAPY_WALLET_STORAGE_CREDS: {}", e))?;
    let account = creds["account"]
        .as_str()
        .ok_or("ACAPY_WALLET_STORAGE_CREDS missing 'account' field")?;
    let password = creds["password"]
        .as_str()
        .ok_or("ACAPY_WALLET_STORAGE_CREDS missing 'password' field")?;

    let store_url = format!(
        "postgres://{}:{}@{}/{}",
        percent_encode(account),
        percent_encode(password),
        db_url,
        percent_encode(&wallet_name),
    );

    let key_method = match key_derivation.as_str() {
        "RAW" => StoreKeyMethod::RawKey,
        "ARGON2I_INT" => StoreKeyMethod::DeriveKey(KdfMethod::Argon2i(Argon2Level::Interactive)),
        _ => StoreKeyMethod::DeriveKey(KdfMethod::Argon2i(Argon2Level::Moderate)),
    };

    println!("Opening Askar store (db={}, key_method={})...", wallet_name, key_derivation);
    let store = Store::open(&store_url, Some(key_method), wallet_key.into(), None)
        .await
        .map_err(|e| format!("Failed to open store: {}", e))?;
    println!("Askar store opened.");
    Ok(store)
}

async fn db_list_profiles(wallet_name: Option<&str>) -> Result<(), String> {
    let store = open_store_from_env(wallet_name, false).await?;
    let profiles = store
        .list_profiles()
        .await
        .map_err(|e| format!("Failed to list profiles: {}", e))?;
    println!("Profiles ({}):", profiles.len());
    for profile in &profiles {
        println!("  {}", profile);
    }
    store.close().await.map_err(|e| format!("Failed to close store: {}", e))?;
    Ok(())
}

/// Known ACA-Py record categories (RECORD_TYPE values from ACA-Py models)
const KNOWN_CATEGORIES: &[&str] = &[
    // Connections
    "connection",
    // Out-of-Band
    "oob-invitation",
    // Issue Credential v1.0 / v2.0
    "credential_exchange_v10",
    "issue-cred-v2.0",
    // Present Proof v1.0 / v2.0
    "presentation_exchange_v10",
    "present-proof-v2.0",
    // Revocation
    "issuer_cred_rev",
    "issuer_rev_reg",
    // DID Exchange
    "did",
    "did_doc",
    // Mediation / Routing
    "mediation_request",
    "route_record",
    "default_mediator",
    "keylist_update_rule",
    // Endorsement
    "endorse_transaction",
    // Basic Message
    "basicmessage",
    // Discover Features
    "discovery_record",
    // Action Menu
    "menu",
];

async fn db_list_categories(wallet_name: Option<&str>, profile: Option<&str>) -> Result<(), String> {
    let store = open_store_from_env(wallet_name, profile.is_some()).await?;
    let mut session = store
        .session(profile.map(|s| s.to_string()))
        .await
        .map_err(|e| format!("Failed to open session: {}", e))?;

    let profile_label = profile.unwrap_or("default");
    println!("Categories in profile '{}':", profile_label);

    let mut found = 0usize;
    for category in KNOWN_CATEGORIES {
        let count = session
            .count(Some(category), None)
            .await
            .map_err(|e| format!("Failed to count category '{}': {}", category, e))?;
        if count > 0 {
            println!("  {} ({})", category, count);
            found += 1;
        }
    }
    println!("Total: {} categories found", found);

    store.close().await.map_err(|e| format!("Failed to close store: {}", e))?;
    Ok(())
}

async fn db_count(wallet_name: Option<&str>, profile: Option<&str>, category: &str) -> Result<(), String> {
    let store = open_store_from_env(wallet_name, profile.is_some()).await?;
    let mut session = store
        .session(profile.map(|s| s.to_string()))
        .await
        .map_err(|e| format!("Failed to open session: {}", e))?;

    let count = session
        .count(Some(category), None)
        .await
        .map_err(|e| format!("Failed to count records: {}", e))?;

    let profile_label = profile.unwrap_or("default");
    println!("Profile '{}', category '{}': {} records", profile_label, category, count);

    store.close().await.map_err(|e| format!("Failed to close store: {}", e))?;
    Ok(())
}

async fn db_delete(wallet_name: Option<&str>, profile: Option<&str>, category: &str, dry_run: bool) -> Result<(), String> {
    let store = open_store_from_env(wallet_name, profile.is_some()).await?;
    let mut session = store
        .session(profile.map(|s| s.to_string()))
        .await
        .map_err(|e| format!("Failed to open session: {}", e))?;

    let count = session
        .count(Some(category), None)
        .await
        .map_err(|e| format!("Failed to count records: {}", e))?;

    let profile_label = profile.unwrap_or("default");

    if count == 0 {
        println!("No records found for category '{}' in profile '{}'.", category, profile_label);
        store.close().await.map_err(|e| format!("Failed to close store: {}", e))?;
        return Ok(());
    }

    if dry_run {
        println!("[DRY-RUN] Would delete {} record(s) of category '{}' in profile '{}'.", count, category, profile_label);
    } else {
        println!("Deleting {} record(s) of category '{}' in profile '{}'...", count, category, profile_label);
        session
            .remove_all(Some(category), None)
            .await
            .map_err(|e| format!("Failed to delete records: {}", e))?;
        println!("Done. Deleted {} record(s).", count);
    }

    store.close().await.map_err(|e| format!("Failed to close store: {}", e))?;
    Ok(())
}

fn percent_encode(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    encoded
}

