use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "acapy-tools")]
#[command(about = "ACA-Py utility tools", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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

/// Shared REST API connection arguments.
#[derive(Args, Debug, Clone)]
pub struct ApiArgs {
    /// Bearer token for Authorization header (optional)
    #[arg(short, long)]
    pub token: Option<String>,

    /// API key for X-API-Key header (optional)
    #[arg(short = 'k', long)]
    pub api_key: Option<String>,

    /// ACA-Py Admin API base URL
    #[arg(short = 'u', long, default_value = "http://localhost:8021")]
    pub base_url: String,
}

/// Batch size argument for paginated operations.
#[derive(Args, Debug, Clone)]
pub struct BatchArgs {
    /// Number of records to fetch per batch
    #[arg(short, long, default_value_t = 5)]
    pub batch_size: u32,
}

/// Arguments specific to delete operations.
#[derive(Args, Debug, Clone)]
pub struct DeleteArgs {
    /// Show records without deleting
    #[arg(short, long)]
    pub dry_run: bool,

    /// Print each deletion result
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum ConnectionCommands {
    /// Delete all connections
    Delete {
        #[command(flatten)]
        api: ApiArgs,
        #[command(flatten)]
        batch: BatchArgs,
        #[command(flatten)]
        del: DeleteArgs,
    },
    /// Count all connections using pagination
    Count {
        #[command(flatten)]
        api: ApiArgs,
        #[command(flatten)]
        batch: BatchArgs,
    },
    /// List connection IDs (limited to 5)
    List {
        #[command(flatten)]
        api: ApiArgs,
    },
}

#[derive(Subcommand, Debug)]
pub enum PresexCommands {
    /// Delete all presentation exchange records
    Delete {
        #[command(flatten)]
        api: ApiArgs,
        #[command(flatten)]
        batch: BatchArgs,
        #[command(flatten)]
        del: DeleteArgs,
    },
    /// Count all presentation exchange records using pagination
    Count {
        #[command(flatten)]
        api: ApiArgs,
        #[command(flatten)]
        batch: BatchArgs,
    },
    /// List presentation exchange IDs (limited to 5)
    List {
        #[command(flatten)]
        api: ApiArgs,
    },
}

#[derive(Subcommand, Debug)]
pub enum CredexCommands {
    /// Delete all credential exchange records
    Delete {
        #[command(flatten)]
        api: ApiArgs,
        #[command(flatten)]
        batch: BatchArgs,
        #[command(flatten)]
        del: DeleteArgs,
    },
    /// Count all credential exchange records using pagination
    Count {
        #[command(flatten)]
        api: ApiArgs,
        #[command(flatten)]
        batch: BatchArgs,
    },
    /// List credential exchange IDs (limited to 5)
    List {
        #[command(flatten)]
        api: ApiArgs,
    },
}

#[derive(Subcommand, Debug)]
pub enum OobCommands {
    /// Delete all out-of-band invitations
    Delete {
        #[command(flatten)]
        api: ApiArgs,
        #[command(flatten)]
        batch: BatchArgs,
        #[command(flatten)]
        del: DeleteArgs,
    },
    /// Count all out-of-band invitations using pagination
    Count {
        #[command(flatten)]
        api: ApiArgs,
        #[command(flatten)]
        batch: BatchArgs,
    },
    /// List out-of-band invitation IDs (limited to 5)
    List {
        #[command(flatten)]
        api: ApiArgs,
    },
}

#[derive(Subcommand, Debug)]
pub enum DbCommands {
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
