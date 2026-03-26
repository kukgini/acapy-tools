use futures::future::join_all;
use serde::Deserialize;

use crate::cli::ApiArgs;
use crate::models::{PagedResponse, Record};

/// Configuration for a REST API resource type.
pub struct ResourceConfig {
    pub list_path: &'static str,
    pub delete_path: &'static str,
    pub label: &'static str,
}

impl ResourceConfig {
    pub const CONNECTION: ResourceConfig = ResourceConfig {
        list_path: "/connections",
        delete_path: "/connections",
        label: "connection",
    };
    pub const PRESEX: ResourceConfig = ResourceConfig {
        list_path: "/present-proof/records",
        delete_path: "/present-proof/records",
        label: "presentation exchange",
    };
    pub const CREDEX: ResourceConfig = ResourceConfig {
        list_path: "/issue-credential/records",
        delete_path: "/issue-credential/records",
        label: "credential exchange",
    };
    pub const OOB: ResourceConfig = ResourceConfig {
        list_path: "/out-of-band/records",
        delete_path: "/out-of-band/invitations",
        label: "OOB invitation",
    };
}

const LIST_LIMIT: u32 = 5;

fn build_client(
    token: Option<&str>,
    api_key: Option<&str>,
) -> Result<reqwest::Client, String> {
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

async fn fetch_page<T: for<'de> Deserialize<'de>>(
    client: &reqwest::Client,
    base_url: &str,
    path: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<T>, reqwest::Error> {
    let url = format!("{}{}?limit={}&offset={}", base_url, path, limit, offset);
    let resp = client.get(&url).send().await?.error_for_status()?;
    let data: PagedResponse<T> = resp.json().await?;
    Ok(data.results)
}

async fn delete_one(
    client: &reqwest::Client,
    base_url: &str,
    path: &str,
    id: &str,
) -> Result<(), reqwest::Error> {
    let url = format!("{}{}/{}", base_url, path, id);
    client.delete(&url).send().await?.error_for_status()?;
    Ok(())
}

// --- Generic orchestration ---

pub async fn run_delete<T: Record>(
    cfg: &ResourceConfig,
    api: &ApiArgs,
    batch_size: u32,
    dry_run: bool,
    verbose: bool,
) -> Result<(), String> {
    let client = build_client(api.token.as_deref(), api.api_key.as_deref())?;
    let base_url = &api.base_url;
    let mut total_deleted = 0u32;
    let mut had_error = false;

    loop {
        // Always offset=0: after deleting a batch, remaining records shift up.
        let records: Vec<T> =
            match fetch_page(&client, base_url, cfg.list_path, batch_size, 0).await {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Failed to fetch {} records: {}", cfg.label, e);
                    had_error = true;
                    break;
                }
            };

        if records.is_empty() {
            println!("No more {} records found.", cfg.label);
            break;
        }

        println!("Found {} {}(s)", records.len(), cfg.label);

        if dry_run {
            for rec in &records {
                let fields = rec.display_fields();
                if fields.is_empty() {
                    let oob = rec.oob_id().unwrap_or("N/A");
                    println!("  [DRY-RUN] Would delete: {} (oob={})", rec.id(), oob);
                } else {
                    let oob_part = rec.oob_id().map(|o| format!(", oob={}", o)).unwrap_or_default();
                    println!(
                        "  [DRY-RUN] Would delete: {} ({}{})",
                        rec.id(),
                        fields,
                        oob_part
                    );
                }
            }
            break;
        }

        // Side-delete associated OOB records in parallel
        let oob_futs: Vec<_> = records
            .iter()
            .filter_map(|r| {
                r.oob_id().map(|oid| {
                    let oid = oid.to_string();
                    let client = &client;
                    async move {
                        let _ =
                            delete_one(client, base_url, "/out-of-band/invitations", &oid).await;
                    }
                })
            })
            .collect();
        join_all(oob_futs).await;

        // Delete main records in parallel
        let del_futs: Vec<_> = records
            .iter()
            .map(|rec| {
                let client = &client;
                async move {
                    match delete_one(client, base_url, cfg.delete_path, rec.id()).await {
                        Ok(()) => {
                            if verbose {
                                let fields = rec.display_fields();
                                if fields.is_empty() {
                                    println!("  Deleted: {}", rec.id());
                                } else {
                                    println!("  Deleted: {} ({})", rec.id(), fields);
                                }
                            }
                            true
                        }
                        Err(e) => {
                            eprintln!("  Failed to delete {}: {}", rec.id(), e);
                            false
                        }
                    }
                }
            })
            .collect();
        let results = join_all(del_futs).await;
        let batch_deleted = results.iter().filter(|&&ok| ok).count() as u32;
        total_deleted += batch_deleted;
        if batch_deleted == 0 {
            eprintln!("Warning: no records deleted in this batch. Stopping.");
            break;
        }
    }

    if dry_run {
        println!("Dry run complete. No {}s were deleted.", cfg.label);
    } else {
        println!("Done. Total deleted: {}", total_deleted);
    }

    if had_error {
        Err(format!("Failed to complete {} deletion", cfg.label))
    } else {
        Ok(())
    }
}

pub async fn run_count<T: Record>(
    cfg: &ResourceConfig,
    api: &ApiArgs,
    batch_size: u32,
) -> Result<(), String> {
    let client = build_client(api.token.as_deref(), api.api_key.as_deref())?;
    let base_url = &api.base_url;
    let mut total_count = 0u32;
    let mut offset = 0u32;

    loop {
        let records: Vec<T> =
            match fetch_page(&client, base_url, cfg.list_path, batch_size, offset).await {
                Ok(r) => r,
                Err(e) => {
                    return Err(format!("Failed to fetch {} records: {}", cfg.label, e));
                }
            };

        let batch_count = records.len() as u32;
        if batch_count == 0 {
            break;
        }

        total_count += batch_count;
        offset += batch_count;

        println!(
            "Fetched {} records (total so far: {})",
            batch_count, total_count
        );

        if batch_count < batch_size {
            break;
        }
    }

    println!("Total {} records: {}", cfg.label, total_count);
    Ok(())
}

pub async fn run_list<T: Record>(
    cfg: &ResourceConfig,
    api: &ApiArgs,
) -> Result<(), String> {
    let client = build_client(api.token.as_deref(), api.api_key.as_deref())?;
    let records: Vec<T> =
        fetch_page(&client, &api.base_url, cfg.list_path, LIST_LIMIT, 0)
            .await
            .map_err(|e| format!("Failed to fetch {} records: {}", cfg.label, e))?;
    for rec in records {
        println!("{}", rec.id());
    }
    Ok(())
}
