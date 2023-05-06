use axum::{
    extract::State,
    response::IntoResponse,
    Json
};
use tokio::sync::{MutexGuard};
use crate::AppState;
use crate::services::knmidata::{InducedBevingenGeoJson, InducedBevingen};


pub async fn induced_bevingen(State(state): State<AppState>) -> impl IntoResponse {
    let data: MutexGuard<InducedBevingen> = state.induced.lock().await;
    Json(data.clone())
}

pub async fn induced_bevingen_geojson(State(state): State<AppState>) -> impl IntoResponse {
    let data: MutexGuard<InducedBevingenGeoJson> = state.induced_geojson.lock().await;
    Json(data.clone())
}
