use serde::Deserialize;
use serde::Serialize;

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, derive_more::Error, derive_more::Display,
)]
#[display("{error_name}: {error}")]
#[serde(rename_all = "camelCase")]
pub struct BitGoApiError {
    /// Human-readable error message
    error: String,
    /// Contains error code
    error_name: String,
    req_id: String,
}
