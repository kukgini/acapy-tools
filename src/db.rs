use aries_askar::storage::{Argon2Level, KdfMethod};
use aries_askar::{Store, StoreKeyMethod};

/// Known ACA-Py record categories (0.8.2 through 1.5.x)
const KNOWN_CATEGORIES: &[&str] = &[
    // Connections
    "connection",
    // Out-of-Band
    "oob_invitation",
    "oob_record",
    // Issue Credential v1.0 / v2.0
    "credential_exchange_v10",
    "cred_ex_v20",
    "indy_cred_ex_v20",
    "ld_proof_cred_ex_v20",
    "anoncreds_cred_ex_v20",
    // Present Proof v1.0 / v2.0
    "presentation_exchange_v10",
    "pres_ex_v20",
    // Revocation (protocol records)
    "issuer_cred_rev",
    "issuer_rev_reg",
    "revocation_notification",
    // Revocation (anoncreds/indy storage)
    "revocation_reg",
    "revocation_reg_def",
    "revocation_reg_def_private",
    "revocation_reg_def_issuer",
    "revocation_reg_info",
    "revocation_list",
    // DID / Keys
    "did",
    "did_doc",
    "did_key",
    "did_rotate",
    "long_peer_did_4_doc",
    // Wallet / Credentials (anoncreds/indy storage)
    "credential",
    "credential_def",
    "credential_def_private",
    "credential_def_key_proof",
    "schema",
    "master_secret",
    "config",
    // Askar internal
    "cryptokey",
    // Mediation / Routing
    "mediation_requests",
    "forward_route",
    // Endorsement
    "transaction",
    // Discovery
    "discovery_exchange_v10",
    "discovery_exchange_v20",
    // Introduction
    "introduction_record",
    // Wallet (multitenancy)
    "wallet_record",
];

/// Parsed wallet configuration from ACA-Py environment variables.
struct WalletConfig {
    store_url: String,
    wallet_key: String,
    key_method: StoreKeyMethod,
    wallet_name: String,
    key_derivation: String,
}

impl WalletConfig {
    fn from_env(
        wallet_name_override: Option<&str>,
        profile: Option<&str>,
    ) -> Result<Self, String> {
        let db_host_url = Self::read_storage_url()?;
        let (account, password) = Self::read_credentials()?;
        let wallet_name = Self::resolve_wallet_name(wallet_name_override, profile)?;
        let wallet_key = std::env::var("ACAPY_WALLET_KEY")
            .map_err(|_| "ACAPY_WALLET_KEY is not set".to_string())?;
        let key_derivation = std::env::var("ACAPY_WALLET_KEY_DERIVATION_METHOD")
            .unwrap_or_else(|_| "ARGON2I_MOD".to_string());
        let key_method = Self::parse_key_method(&key_derivation);
        let store_url = format!(
            "postgres://{}:{}@{}/{}",
            percent_encode(&account),
            percent_encode(&password),
            db_host_url,
            percent_encode(&wallet_name),
        );
        Ok(WalletConfig {
            store_url,
            wallet_key,
            key_method,
            wallet_name,
            key_derivation,
        })
    }

    fn read_storage_url() -> Result<String, String> {
        let raw = std::env::var("ACAPY_WALLET_STORAGE_CONFIG")
            .map_err(|_| "ACAPY_WALLET_STORAGE_CONFIG is not set".to_string())?;
        let config: serde_json::Value = serde_json::from_str(&raw)
            .map_err(|e| format!("Failed to parse ACAPY_WALLET_STORAGE_CONFIG: {}", e))?;
        config["url"]
            .as_str()
            .ok_or("ACAPY_WALLET_STORAGE_CONFIG missing 'url' field".to_string())
            .map(|s| s.to_string())
    }

    fn read_credentials() -> Result<(String, String), String> {
        let raw = std::env::var("ACAPY_WALLET_STORAGE_CREDS")
            .map_err(|_| "ACAPY_WALLET_STORAGE_CREDS is not set".to_string())?;
        let creds: serde_json::Value = serde_json::from_str(&raw)
            .map_err(|e| format!("Failed to parse ACAPY_WALLET_STORAGE_CREDS: {}", e))?;
        let account = creds["account"]
            .as_str()
            .ok_or("ACAPY_WALLET_STORAGE_CREDS missing 'account' field")?
            .to_string();
        let password = creds["password"]
            .as_str()
            .ok_or("ACAPY_WALLET_STORAGE_CREDS missing 'password' field")?
            .to_string();
        Ok((account, password))
    }

    /// Resolves the wallet database name using one of three strategies:
    /// 1. Explicit override via --wallet-name
    /// 2. Multitenancy config (when --profile is specified, reads ACAPY_MULTITENANCY_CONFIGURATION)
    /// 3. Default single-tenant (reads ACAPY_WALLET_NAME)
    fn resolve_wallet_name(
        wallet_name_override: Option<&str>,
        profile: Option<&str>,
    ) -> Result<String, String> {
        if let Some(name) = wallet_name_override {
            return Ok(name.to_string());
        }
        if profile.is_some() {
            let mt_config = std::env::var("ACAPY_MULTITENANCY_CONFIGURATION")
                .map_err(|_| "ACAPY_MULTITENANCY_CONFIGURATION is not set".to_string())?;
            let mt: serde_json::Value = serde_json::from_str(&mt_config)
                .map_err(|e| format!("Failed to parse ACAPY_MULTITENANCY_CONFIGURATION: {}", e))?;
            mt["wallet_name"]
                .as_str()
                .ok_or(
                    "ACAPY_MULTITENANCY_CONFIGURATION missing 'wallet_name' field".to_string(),
                )
                .map(|s| s.to_string())
        } else {
            std::env::var("ACAPY_WALLET_NAME")
                .map_err(|_| "ACAPY_WALLET_NAME is not set".to_string())
        }
    }

    fn parse_key_method(key_derivation: &str) -> StoreKeyMethod {
        match key_derivation {
            "RAW" => StoreKeyMethod::RawKey,
            "ARGON2I_INT" => {
                StoreKeyMethod::DeriveKey(KdfMethod::Argon2i(Argon2Level::Interactive))
            }
            _ => StoreKeyMethod::DeriveKey(KdfMethod::Argon2i(Argon2Level::Moderate)),
        }
    }
}

async fn open_store(cfg: &WalletConfig) -> Result<Store, String> {
    println!(
        "Opening Askar store (db={}, key_method={})...",
        cfg.wallet_name, cfg.key_derivation
    );
    let store = Store::open(
        &cfg.store_url,
        Some(cfg.key_method.clone()),
        cfg.wallet_key.clone().into(),
        None,
    )
    .await
    .map_err(|e| format!("Failed to open store: {}", e))?;
    println!("Askar store opened.");
    Ok(store)
}

/// RFC 3986 percent-encoding for unreserved characters only.
/// Hand-rolled to avoid adding a crate dependency for this single use.
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

// --- Public DB command handlers ---

pub async fn db_list_profiles(wallet_name: Option<&str>) -> Result<(), String> {
    let cfg = WalletConfig::from_env(wallet_name, None)?;
    let store = open_store(&cfg).await?;
    let profiles = store
        .list_profiles()
        .await
        .map_err(|e| format!("Failed to list profiles: {}", e))?;
    println!("Profiles ({}):", profiles.len());
    for profile in &profiles {
        println!("  {}", profile);
    }
    Ok(())
}

pub async fn db_list_categories(
    wallet_name: Option<&str>,
    profile: Option<&str>,
) -> Result<(), String> {
    let cfg = WalletConfig::from_env(wallet_name, profile)?;
    let store = open_store(&cfg).await?;

    let profiles: Vec<String> = if let Some(p) = profile {
        vec![p.to_string()]
    } else {
        store
            .list_profiles()
            .await
            .map_err(|e| format!("Failed to list profiles: {}", e))?
    };

    for profile_name in &profiles {
        let mut session = store
            .session(Some(profile_name.clone()))
            .await
            .map_err(|e| format!("Failed to open session for '{}': {}", profile_name, e))?;

        let mut found = 0usize;
        let mut results = Vec::new();
        for category in KNOWN_CATEGORIES {
            let count = session
                .count(Some(category), None)
                .await
                .map_err(|e| format!("Failed to count category '{}': {}", category, e))?;
            if count > 0 {
                results.push((*category, count));
                found += 1;
            }
        }

        if found > 0 {
            println!("Profile '{}':", profile_name);
            for (category, count) in &results {
                println!("  {} ({})", category, count);
            }
            println!("  ({} categories)\n", found);
        }
    }

    Ok(())
}

pub async fn db_count(
    wallet_name: Option<&str>,
    profile: Option<&str>,
    category: &str,
) -> Result<(), String> {
    let cfg = WalletConfig::from_env(wallet_name, profile)?;
    let store = open_store(&cfg).await?;
    let mut session = store
        .session(profile.map(|s| s.to_string()))
        .await
        .map_err(|e| format!("Failed to open session: {}", e))?;

    let count = session
        .count(Some(category), None)
        .await
        .map_err(|e| format!("Failed to count records: {}", e))?;

    let profile_label = profile.unwrap_or("default");
    println!(
        "Profile '{}', category '{}': {} records",
        profile_label, category, count
    );

    Ok(())
}

pub async fn db_delete(
    wallet_name: Option<&str>,
    profile: Option<&str>,
    category: &str,
    dry_run: bool,
) -> Result<(), String> {
    let cfg = WalletConfig::from_env(wallet_name, profile)?;
    let store = open_store(&cfg).await?;
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
        println!(
            "No records found for category '{}' in profile '{}'.",
            category, profile_label
        );
        return Ok(());
    }

    if dry_run {
        println!(
            "[DRY-RUN] Would delete {} record(s) of category '{}' in profile '{}'.",
            count, category, profile_label
        );
    } else {
        println!(
            "Deleting {} record(s) of category '{}' in profile '{}'...",
            count, category, profile_label
        );
        session
            .remove_all(Some(category), None)
            .await
            .map_err(|e| format!("Failed to delete records: {}", e))?;
        println!("Done. Deleted {} record(s).", count);
    }
    Ok(())
}
