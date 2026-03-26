use serde::Deserialize;

/// Generic paged response — all ACA-Py list endpoints return `{ "results": [...] }`.
#[derive(Debug, Deserialize)]
pub struct PagedResponse<T> {
    pub results: Vec<T>,
}

/// Trait implemented by each ACA-Py record type to enable generic batch operations.
pub trait Record: for<'de> Deserialize<'de> + Send + Sync {
    /// The primary ID field used for deletion.
    fn id(&self) -> &str;

    /// Optional OOB invitation ID for side-deletion.
    /// Returns `None` by default; overridden by types that link to OOB records.
    fn oob_id(&self) -> Option<&str> {
        None
    }

    /// Fields to display in dry-run and verbose output.
    fn display_fields(&self) -> String;
}

#[derive(Debug, Deserialize)]
pub struct Connection {
    pub connection_id: String,
    pub state: Option<String>,
    pub their_label: Option<String>,
    pub invitation_msg_id: Option<String>,
}

impl Record for Connection {
    fn id(&self) -> &str {
        &self.connection_id
    }

    fn oob_id(&self) -> Option<&str> {
        self.invitation_msg_id.as_deref()
    }

    fn display_fields(&self) -> String {
        format!(
            "state={}, label={}",
            self.state.as_deref().unwrap_or("unknown"),
            self.their_label.as_deref().unwrap_or("N/A")
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct PresentationExchange {
    pub presentation_exchange_id: String,
    pub state: Option<String>,
    pub connection_id: Option<String>,
    #[serde(default)]
    pub parent_thread_id: Option<String>,
}

impl Record for PresentationExchange {
    fn id(&self) -> &str {
        &self.presentation_exchange_id
    }

    fn oob_id(&self) -> Option<&str> {
        self.parent_thread_id.as_deref()
    }

    fn display_fields(&self) -> String {
        format!(
            "state={}, connection_id={}",
            self.state.as_deref().unwrap_or("unknown"),
            self.connection_id.as_deref().unwrap_or("N/A")
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct CredentialExchange {
    pub credential_exchange_id: String,
    #[serde(default)]
    pub parent_thread_id: Option<String>,
}

impl Record for CredentialExchange {
    fn id(&self) -> &str {
        &self.credential_exchange_id
    }

    fn oob_id(&self) -> Option<&str> {
        self.parent_thread_id.as_deref()
    }

    fn display_fields(&self) -> String {
        String::new()
    }
}

#[derive(Debug, Deserialize)]
pub struct OobInvitation {
    pub oob_id: String,
    pub invi_msg_id: Option<String>,
    pub state: Option<String>,
}

impl Record for OobInvitation {
    fn id(&self) -> &str {
        &self.oob_id
    }

    fn display_fields(&self) -> String {
        format!(
            "state={}, invi_msg_id={}",
            self.state.as_deref().unwrap_or("unknown"),
            self.invi_msg_id.as_deref().unwrap_or("N/A")
        )
    }
}
