use clap::{Parser, Subcommand};
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

fn build_client(token: Option<&str>, api_key: Option<&str>) -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    if let Some(t) = token {
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", t).parse().unwrap(),
        );
    }
    if let Some(key) = api_key {
        headers.insert(
            reqwest::header::HeaderName::from_static("x-api-key"),
            key.parse().unwrap(),
        );
    }
    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
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

async fn delete_oob_invitation(
    client: &reqwest::Client,
    base_url: &str,
    invi_msg_id: &str,
) -> Result<(), reqwest::Error> {
    let url = format!("{}/out-of-band/invitations/{}", base_url, invi_msg_id);
    client.delete(&url).send().await?.error_for_status()?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Connection { action } => match action {
            ConnectionCommands::Delete {
                token,
                api_key,
                base_url,
                batch_size,
                dry_run,
            } => {
                delete_all_connections(token.as_deref(), api_key.as_deref(), &base_url, batch_size, dry_run)
                    .await;
            }
            ConnectionCommands::Count {
                token,
                api_key,
                base_url,
                batch_size,
            } => {
                count_connections(token.as_deref(), api_key.as_deref(), &base_url, batch_size).await;
            }
            ConnectionCommands::List {
                token,
                api_key,
                base_url,
            } => {
                list_connections(token.as_deref(), api_key.as_deref(), &base_url).await;
            }
        },
        Commands::Presex { action } => match action {
            PresexCommands::Delete {
                token,
                api_key,
                base_url,
                batch_size,
                dry_run,
            } => {
                delete_all_presex(token.as_deref(), api_key.as_deref(), &base_url, batch_size, dry_run)
                    .await;
            }
            PresexCommands::Count {
                token,
                api_key,
                base_url,
                batch_size,
            } => {
                count_presex(token.as_deref(), api_key.as_deref(), &base_url, batch_size).await;
            }
            PresexCommands::List {
                token,
                api_key,
                base_url,
            } => {
                list_presex(token.as_deref(), api_key.as_deref(), &base_url).await;
            }
        },
        Commands::Credex { action } => match action {
            CredexCommands::Delete {
                token,
                api_key,
                base_url,
                batch_size,
                dry_run,
            } => {
                delete_all_credex(token.as_deref(), api_key.as_deref(), &base_url, batch_size, dry_run)
                    .await;
            }
            CredexCommands::Count {
                token,
                api_key,
                base_url,
                batch_size,
            } => {
                count_credex(token.as_deref(), api_key.as_deref(), &base_url, batch_size).await;
            }
            CredexCommands::List {
                token,
                api_key,
                base_url,
            } => {
                list_credex(token.as_deref(), api_key.as_deref(), &base_url).await;
            }
        },
    }
}

async fn delete_all_connections(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
    batch_size: u32,
    dry_run: bool,
) {
    let client = build_client(token, api_key);
    let mut total_deleted = 0u32;

    loop {
        let connections = match get_connections(&client, base_url, batch_size, 0).await {
            Ok(conns) => conns,
            Err(e) => {
                eprintln!("Failed to fetch connections: {}", e);
                break;
            }
        };

        if connections.is_empty() {
            println!("No more connections found.");
            break;
        }

        println!("Found {} connection(s)", connections.len());

        for conn in connections {
            let state = conn.state.as_deref().unwrap_or("unknown");
            let label = conn.their_label.as_deref().unwrap_or("N/A");
            let invi_msg_id = conn.invitation_msg_id.as_deref();

            if dry_run {
                println!(
                    "  [DRY-RUN] Would delete: {} (state={}, label={}, oob={})",
                    conn.connection_id, state, label, invi_msg_id.unwrap_or("N/A")
                );
            } else {
                // Delete OOB first if exists
                if let Some(oob_id) = invi_msg_id {
                    match delete_oob_invitation(&client, base_url, oob_id).await {
                        Ok(()) => println!("    Deleted OOB: {}", oob_id),
                        Err(_) => {} // OOB may not exist or already deleted
                    }
                }
                match delete_connection(&client, base_url, &conn.connection_id).await {
                    Ok(()) => {
                        println!(
                            "  Deleted: {} (state={}, label={})",
                            conn.connection_id, state, label
                        );
                        total_deleted += 1;
                    }
                    Err(e) => {
                        eprintln!("  Failed to delete {}: {}", conn.connection_id, e);
                    }
                }
            }
        }
    }

    if dry_run {
        println!("Dry run complete. No connections were deleted.");
    } else {
        println!("Done. Total deleted: {}", total_deleted);
    }
}

async fn count_connections(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
    batch_size: u32,
) {
    let client = build_client(token, api_key);
    let mut total_count = 0u32;
    let mut offset = 0u32;

    loop {
        let records = match get_connections(&client, base_url, batch_size, offset).await {
            Ok(recs) => recs,
            Err(e) => {
                eprintln!("Failed to fetch connections: {}", e);
                break;
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
}

async fn delete_all_presex(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
    batch_size: u32,
    dry_run: bool,
) {
    let client = build_client(token, api_key);
    let mut total_deleted = 0u32;

    loop {
        let records = match get_presentation_exchanges(&client, base_url, batch_size, 0).await {
            Ok(recs) => recs,
            Err(e) => {
                eprintln!("Failed to fetch presentation exchanges: {}", e);
                break;
            }
        };

        if records.is_empty() {
            println!("No more presentation exchange records found.");
            break;
        }

        println!("Found {} presentation exchange record(s)", records.len());

        for rec in records {
            let state = rec.state.as_deref().unwrap_or("unknown");
            let conn_id = rec.connection_id.as_deref().unwrap_or("N/A");
            let parent_thread = rec.parent_thread_id.as_deref();

            if dry_run {
                println!(
                    "  [DRY-RUN] Would delete: {} (state={}, connection_id={}, oob={})",
                    rec.presentation_exchange_id, state, conn_id, parent_thread.unwrap_or("N/A")
                );
            } else {
                // Delete OOB first if exists
                if let Some(oob_id) = parent_thread {
                    match delete_oob_invitation(&client, base_url, oob_id).await {
                        Ok(()) => println!("    Deleted OOB: {}", oob_id),
                        Err(_) => {} // OOB may not exist or already deleted
                    }
                }
                match delete_presentation_exchange(&client, base_url, &rec.presentation_exchange_id).await {
                    Ok(()) => {
                        println!(
                            "  Deleted: {} (state={}, connection_id={})",
                            rec.presentation_exchange_id, state, conn_id
                        );
                        total_deleted += 1;
                    }
                    Err(e) => {
                        eprintln!("  Failed to delete {}: {}", rec.presentation_exchange_id, e);
                    }
                }
            }
        }
    }

    if dry_run {
        println!("Dry run complete. No presentation exchanges were deleted.");
    } else {
        println!("Done. Total deleted: {}", total_deleted);
    }
}

async fn count_presex(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
    batch_size: u32,
) {
    let client = build_client(token, api_key);
    let mut total_count = 0u32;
    let mut offset = 0u32;

    loop {
        let records = match get_presentation_exchanges(&client, base_url, batch_size, offset).await {
            Ok(recs) => recs,
            Err(e) => {
                eprintln!("Failed to fetch presentation exchanges: {}", e);
                break;
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
}

async fn count_credex(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
    batch_size: u32,
) {
    let client = build_client(token, api_key);
    let mut total_count = 0u32;
    let mut offset = 0u32;

    loop {
        let records = match get_credential_exchanges(&client, base_url, batch_size, offset).await {
            Ok(recs) => recs,
            Err(e) => {
                eprintln!("Failed to fetch credential exchanges: {}", e);
                break;
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
}

async fn delete_all_credex(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
    batch_size: u32,
    dry_run: bool,
) {
    let client = build_client(token, api_key);
    let mut total_deleted = 0u32;

    loop {
        let records = match get_credential_exchanges(&client, base_url, batch_size, 0).await {
            Ok(recs) => recs,
            Err(e) => {
                eprintln!("Failed to fetch credential exchanges: {}", e);
                break;
            }
        };

        if records.is_empty() {
            println!("No more credential exchange records found.");
            break;
        }

        println!("Found {} credential exchange record(s)", records.len());

        for rec in records {
            let parent_thread = rec.parent_thread_id.as_deref();

            if dry_run {
                println!(
                    "  [DRY-RUN] Would delete: {} (oob={})",
                    rec.credential_exchange_id, parent_thread.unwrap_or("N/A")
                );
            } else {
                // Delete OOB first if exists
                if let Some(oob_id) = parent_thread {
                    match delete_oob_invitation(&client, base_url, oob_id).await {
                        Ok(()) => println!("    Deleted OOB: {}", oob_id),
                        Err(_) => {} // OOB may not exist or already deleted
                    }
                }
                match delete_credential_exchange(&client, base_url, &rec.credential_exchange_id).await {
                    Ok(()) => {
                        println!("  Deleted: {}", rec.credential_exchange_id);
                        total_deleted += 1;
                    }
                    Err(e) => {
                        eprintln!("  Failed to delete {}: {}", rec.credential_exchange_id, e);
                    }
                }
            }
        }
    }

    if dry_run {
        println!("Dry run complete. No credential exchanges were deleted.");
    } else {
        println!("Done. Total deleted: {}", total_deleted);
    }
}

async fn list_connections(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
) {
    let client = build_client(token, api_key);
    let limit = 5;

    let connections = match get_connections(&client, base_url, limit, 0).await {
        Ok(conns) => conns,
        Err(e) => {
            eprintln!("Failed to fetch connections: {}", e);
            return;
        }
    };

    for conn in connections {
        println!("{}", conn.connection_id);
    }
}

async fn list_presex(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
) {
    let client = build_client(token, api_key);
    let limit = 5;

    let records = match get_presentation_exchanges(&client, base_url, limit, 0).await {
        Ok(recs) => recs,
        Err(e) => {
            eprintln!("Failed to fetch presentation exchanges: {}", e);
            return;
        }
    };

    for rec in records {
        println!("{}", rec.presentation_exchange_id);
    }
}

async fn list_credex(
    token: Option<&str>,
    api_key: Option<&str>,
    base_url: &str,
) {
    let client = build_client(token, api_key);
    let limit = 5;

    let records = match get_credential_exchanges(&client, base_url, limit, 0).await {
        Ok(recs) => recs,
        Err(e) => {
            eprintln!("Failed to fetch credential exchanges: {}", e);
            return;
        }
    };

    for rec in records {
        println!("{}", rec.credential_exchange_id);
    }
}
