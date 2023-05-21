use axum::extract::Query;
use axum::{
    extract::State,
    response::IntoResponse,
    Json
};
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use serde::Deserialize;
use std::sync::Arc;
use crate::AppState;
use crate::services::knmidata::{InducedBevingenGeoJson, InducedBevingen};

#[derive(Deserialize)]
pub struct QueryParams {
    pub start_year: i32,
    pub end_year: i32,
}

pub async fn induced_bevingen(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let data: InducedBevingen = state.induced.read().clone();
    Json(data)
}

pub async fn induced_bevingen_geojson(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let data: InducedBevingenGeoJson = state.induced_geojson.read().clone();
    Json(data)
}

pub async fn induced_bevingen_geojson_query(State(state): State<Arc<AppState>>, query_params: Query<QueryParams>) -> impl IntoResponse {
    let data = state.induced_geojson.read().clone();
    let mut query_data: InducedBevingenGeoJson = InducedBevingenGeoJson{
        type_: "FeatureCollection".to_string(),
        features: vec![]
    };
    let ds = NaiveDate::from_ymd_opt(query_params.start_year, 1, 1).unwrap();
    let ts = NaiveTime::from_hms_opt(0,0,0).unwrap();
    let start_year = NaiveDateTime::new(ds, ts);

    let de = NaiveDate::from_ymd_opt(query_params.end_year, 12, 31).unwrap();
    let te = NaiveTime::from_hms_opt(23,59,59).unwrap();
    let end_year = NaiveDateTime::new(de, te);

    query_data.features = data.features
        .into_iter()
        .filter(|f| f.datetime >= start_year && f.datetime <= end_year)
        .collect();
    
    Json(query_data)
}   