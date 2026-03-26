mod api;
mod cli;
mod db;
mod models;

use api::ResourceConfig;
use clap::Parser;
use cli::*;
use models::*;

#[tokio::main]
async fn main() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install default CryptoProvider");

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Connection { action } => match action {
            ConnectionCommands::Delete { api, batch, del } => {
                api::run_delete::<Connection>(&ResourceConfig::CONNECTION, &api, batch.batch_size, del.dry_run, del.verbose).await
            }
            ConnectionCommands::Count { api, batch } => {
                api::run_count::<Connection>(&ResourceConfig::CONNECTION, &api, batch.batch_size).await
            }
            ConnectionCommands::List { api } => {
                api::run_list::<Connection>(&ResourceConfig::CONNECTION, &api).await
            }
        },
        Commands::Presex { action } => match action {
            PresexCommands::Delete { api, batch, del } => {
                api::run_delete::<PresentationExchange>(&ResourceConfig::PRESEX, &api, batch.batch_size, del.dry_run, del.verbose).await
            }
            PresexCommands::Count { api, batch } => {
                api::run_count::<PresentationExchange>(&ResourceConfig::PRESEX, &api, batch.batch_size).await
            }
            PresexCommands::List { api } => {
                api::run_list::<PresentationExchange>(&ResourceConfig::PRESEX, &api).await
            }
        },
        Commands::Credex { action } => match action {
            CredexCommands::Delete { api, batch, del } => {
                api::run_delete::<CredentialExchange>(&ResourceConfig::CREDEX, &api, batch.batch_size, del.dry_run, del.verbose).await
            }
            CredexCommands::Count { api, batch } => {
                api::run_count::<CredentialExchange>(&ResourceConfig::CREDEX, &api, batch.batch_size).await
            }
            CredexCommands::List { api } => {
                api::run_list::<CredentialExchange>(&ResourceConfig::CREDEX, &api).await
            }
        },
        Commands::Oob { action } => match action {
            OobCommands::Delete { api, batch, del } => {
                api::run_delete::<OobInvitation>(&ResourceConfig::OOB, &api, batch.batch_size, del.dry_run, del.verbose).await
            }
            OobCommands::Count { api, batch } => {
                api::run_count::<OobInvitation>(&ResourceConfig::OOB, &api, batch.batch_size).await
            }
            OobCommands::List { api } => {
                api::run_list::<OobInvitation>(&ResourceConfig::OOB, &api).await
            }
        },
        Commands::Db { action } => match action {
            DbCommands::ListProfiles { wallet_name } => {
                db::db_list_profiles(wallet_name.as_deref()).await
            }
            DbCommands::ListCategories { wallet_name, profile } => {
                db::db_list_categories(wallet_name.as_deref(), profile.as_deref()).await
            }
            DbCommands::Count { wallet_name, profile, category } => {
                db::db_count(wallet_name.as_deref(), profile.as_deref(), &category).await
            }
            DbCommands::Delete { wallet_name, profile, category, dry_run } => {
                db::db_delete(wallet_name.as_deref(), profile.as_deref(), &category, dry_run).await
            }
        },
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
