pub mod services;
pub mod conversion;
pub mod api;
use axum::{
    error_handling::HandleErrorLayer,
    routing::get,
    http::StatusCode,
    Router,
};
use tower::{BoxError, ServiceBuilder};
use axum::http::{Method};
use tower_http::trace::TraceLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::sync::{Mutex};
use std::{
    time::Duration,
    sync::Arc, net::SocketAddr
};

use crate::services::knmidata::{InducedBevingen, InducedBevingenGeoJson, init_knmi_bevingen};
use crate::api::induced::{induced_bevingen, induced_bevingen_geojson, induced_bevingen_geojson_query};

#[derive(Clone)]
pub struct AppState {
    induced: Arc<Mutex<InducedBevingen>>,
    induced_geojson: Arc<Mutex<InducedBevingenGeoJson>>
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

    // cors middleware
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);

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
        .route("/api/induced/geojson/query", get(induced_bevingen_geojson_query))
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
                .layer(cors)
                .into_inner(),
        )
        .with_state(state);

    // host en poort vanuit config bestand
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    tracing::info!("listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}