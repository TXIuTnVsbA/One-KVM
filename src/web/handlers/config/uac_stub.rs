use axum::{extract::State, Json};
use std::sync::Arc;

use crate::config::UacConfig;
use crate::error::Result;
use crate::state::AppState;

pub async fn get_uac_config(State(state): State<Arc<AppState>>) -> Json<UacConfig> {
    Json(state.config.get().uac.clone())
}

pub async fn update_uac_config(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UacConfig>,
) -> Result<Json<UacConfig>> {
    // On non-Unix platforms we don't have OTG to apply; persist the setting so frontend sees updated config
    state.config.update(|cfg| cfg.uac = request.clone()).await?;
    Ok(Json(state.config.get().uac.clone()))
}
