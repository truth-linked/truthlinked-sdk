use serde::{Deserialize, Serialize};

/// License tier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    Free,
    Professional,
    Enterprise,
    Government,
}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Token exchange request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRequest {
    pub sso_token: String,
    pub requested_scope: Vec<String>,
    pub nonce: String,
    pub channel_binding: String,
}

/// Token exchange response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub af_token: String,
    pub granted_scope: Vec<String>,
    pub expires_at: u64,
    pub exchange_id: String,
}

/// Token validation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateResponse {
    pub valid: bool,
    pub subject: Option<String>,
    pub scope: Option<Vec<String>>,
}

/// Shadow decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowDecision {
    pub divergence_id: String,
    pub iam_allowed: bool,
    pub af_would_allow: bool,
    pub breach_prevented: bool,
}

/// Shadow replay request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayRequest {
    pub logs: Vec<String>,
    pub adapter: String,
}

/// Shadow replay response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayResponse {
    pub events_processed: u64,
    pub breaches_prevented: u64,
    pub false_positives_avoided: u64,
}

/// SOX compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoxReport {
    pub period: String,
    pub total_events: u64,
    pub audit_trail_complete: bool,
    pub no_gaps: bool,
}

/// PCI-DSS compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciReport {
    pub period: String,
    pub access_controls_enforced: bool,
    pub encryption_verified: bool,
    pub audit_complete: bool,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub timestamp: u64,
    pub event_type: String,
    pub subject: String,
    pub action: String,
    pub result: String,
}

/// Usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageResponse {
    pub tier: String,
    pub usage: u32,
    pub limit: u32,
    pub percentage: f32,
    pub days_remaining: i64,
}
