use axum::{extract::State, Json};
use serde_json::json;
use std::sync::Arc;

use crate::config::OtgNetworkConfig;
use crate::error::Result;
use crate::state::AppState;

pub async fn get_otg_network_config(State(state): State<Arc<AppState>>) -> Json<OtgNetworkConfig> {
    Json(state.config.get().otg_network.clone())
}

pub async fn update_otg_network_config(
    State(state): State<Arc<AppState>>,
    Json(request): Json<OtgNetworkConfig>,
) -> Result<Json<OtgNetworkConfig>> {
    state.config.update(|cfg| cfg.otg_network = request.clone()).await?;
    Ok(Json(state.config.get().otg_network.clone()))
}

pub async fn get_otg_network_status(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(json!({"health": "healthy", "error": null}))
}
