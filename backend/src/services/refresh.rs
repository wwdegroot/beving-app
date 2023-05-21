use crate::AppState;
use crate::services::knmidata::{init_knmi_bevingen, InducedBevingen};
use tokio::{time};
use tokio_stream::{StreamExt, wrappers::IntervalStream};
use std::sync::Arc;
use std::time::Duration;
use axum::extract::State;
use tracing::{info, trace};

/// update knmi dataset by doing a new request and writing it to the appstate.
pub async fn refresh_knmi_data(State(state): State<Arc<AppState>>) {
    
    let mut refresh_interval_timer = IntervalStream::new(time::interval(Duration::from_secs(3600)));


    while let Some(_ts) = refresh_interval_timer.next().await {
        info!("refreshing knmi data");

        let refresh_bevingen_data = match init_knmi_bevingen()
        .await {
            Ok(induced_bevingen_data) => {
                trace!("{:?}", induced_bevingen_data);
                induced_bevingen_data
            }
            Err(err) => {
                tracing::error!("Error getting knmi data: {}", err.to_string());
                InducedBevingen{events: vec![]}
            }
        };
        // check if there is data returned else do nothing
        if refresh_bevingen_data.events.len() > 0 {
            // update induced data in appstate 
            let induced = state.induced.try_write();
            match induced {
                Some(mut induced) => {
                    *induced = refresh_bevingen_data.clone()
                }
                None => {
                    tracing::error!("Failed to get write lock on induced appstate RwLock.");
                }
            }
            // update induced_geojson data in appstate 
            let induced_geojson = state.induced_geojson.try_write();
            match induced_geojson {
                Some(mut induced_geojson) => {
                    *induced_geojson = refresh_bevingen_data.into()
                }
                None => {
                    tracing::error!("Failed to get write lock on induced_geojson appstate RwLock.");
                }
            }
        }

        }
}
