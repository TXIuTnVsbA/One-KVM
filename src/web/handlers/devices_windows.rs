use axum::Json;

#[cfg(not(unix))]
use serde_json::Value;

#[cfg(not(unix))]
pub async fn list_network_interfaces() -> Json<Vec<Value>> {
    Json(vec![])
}
