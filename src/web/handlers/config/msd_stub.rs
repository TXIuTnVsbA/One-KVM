#[cfg(not(unix))]
use axum::{extract::State, Json};
#[cfg(not(unix))]
use std::sync::Arc;
#[cfg(not(unix))]
use crate::config::MsdConfig;
#[cfg(not(unix))]
use crate::error::Result;
#[cfg(not(unix))]
use crate::state::AppState;

#[cfg(not(unix))]
pub async fn get_msd_config(State(state): State<Arc<AppState>>) -> Json<MsdConfig> {
    Json(state.config.get().msd.clone())
}

#[cfg(not(unix))]
pub async fn update_msd_config(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MsdConfig>,
) -> Result<Json<MsdConfig>> {
    state.config.update(|cfg| cfg.msd = req.clone()).await?;
    Ok(Json(state.config.get().msd.clone()))
}
