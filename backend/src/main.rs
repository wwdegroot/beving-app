pub mod services;
pub mod conversion;
use crate::services::knmidata::{InducedBevingen, InducedBevingenGeoJson, init_knmi_bevingen};
use axum::{
    error_handling::HandleErrorLayer,
    routing::get,
    http::StatusCode,
    Router,
    extract::State,
    Json,
    response::IntoResponse,
};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use tokio::sync::{Mutex, MutexGuard};
use std::{
    time::Duration,
    sync::Arc, net::SocketAddr
};

#[derive(Clone)]
struct AppState {
    induced: Arc<Mutex<InducedBevingen>>,
    induced_geojson: Arc<Mutex<InducedBevingenGeoJson>>
}


async fn induced_bevingen(State(state): State<AppState>) -> impl IntoResponse {
    let data: MutexGuard<InducedBevingen> = state.induced.lock().await;
    Json(data.clone())
}

async fn induced_bevingen_geojson(State(state): State<AppState>) -> impl IntoResponse {
    let data: MutexGuard<InducedBevingenGeoJson> = state.induced_geojson.lock().await;
    Json(data.clone())
}


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "beving-backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let induced_bevingen_data = init_knmi_bevingen()
        .await
        .expect("Induced beving data niet kunnen laden");
    let state = AppState{
        induced: Arc::new(Mutex::new(induced_bevingen_data.clone())),
        induced_geojson: Arc::new(Mutex::new(induced_bevingen_data.into()))
    };

    let app = Router::new()
        .route("/api/induced", get(induced_bevingen))
        .route("/api/induced/geojson", get(induced_bevingen_geojson))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(state);

    // host en poort vanuit config bestand
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}