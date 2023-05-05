use serde::{de, Serialize, Deserialize, Deserializer};
use serde_json::Value;
use reqwest;

use crate::conversion::rijksdriehoekstelsel::WGS84Coordinate;

#[derive(Serialize, Deserialize, Clone)]
pub struct InducedBeving {
    pub date: String,
    #[serde(deserialize_with = "de_f64_or_string_as_f64")]
    pub depth: f64,
    #[serde(rename = "evaluationMode")]
    pub evaluation_mode: String,
    #[serde(deserialize_with = "de_f64_or_string_as_f64")]
    pub lat: f64,
    #[serde(deserialize_with = "de_f64_or_string_as_f64")]
    pub lon: f64,
    #[serde(deserialize_with = "de_f64_or_string_as_f64")]
    pub mag: f64,
    pub place: String,
    pub time: String,
    #[serde(rename = "type")]
    pub type_: String,
}


fn de_f64_or_string_as_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
      Value::String(s) => s.parse().map_err(de::Error::custom)?,
      Value::Number(num) => num.as_f64().ok_or_else(|| de::Error::custom("Invalid number"))?,
      _ => return Err(de::Error::custom("wrong type")),
    })
  }

#[derive(Serialize, Deserialize, Clone)]
pub struct InducedBevingen {
    pub events: Vec<InducedBeving>,
}

#[derive(Serialize, Deserialize, Clone)] 
pub struct PointRD {
    #[serde(rename = "type")]
    pub type_: String,
    pub coordinates: [f64; 2],
}

impl From<WGS84Coordinate> for PointRD{
    fn from(wgs84: WGS84Coordinate) -> Self {
        let rdnew = wgs84.to_rdnew();
        PointRD { type_: "Point".to_owned(), coordinates: [rdnew.x, rdnew.y] }
    }
}

#[derive(Serialize, Deserialize, Clone)] 
pub struct InducedProperties {
    pub date: String,
    pub depth: f64,
    #[serde(rename = "evaluationMode")]
    pub evaluation_mode: String,
    pub mag: f64,
    pub place: String,
    pub time: String,
}


#[derive(Serialize, Deserialize, Clone)] 
pub struct InducedBevingGeoJson {
    #[serde(rename = "type")]
    pub type_: String,
    pub geometry: PointRD,
    pub properties: InducedProperties,

}

impl From<InducedBeving> for InducedBevingGeoJson {
    fn from(induced_beving: InducedBeving) -> Self {
        let wgs84: WGS84Coordinate = WGS84Coordinate::new(induced_beving.lat, induced_beving.lon);
        let rd_geometry: PointRD = wgs84.into();
        let props = InducedProperties{
            date: induced_beving.date,
            depth: induced_beving.depth,
            evaluation_mode: induced_beving.evaluation_mode,
            mag: induced_beving.mag,
            place: induced_beving.place,
            time: induced_beving.time,
        };
        InducedBevingGeoJson{
            type_: "Feature".to_string(),
            geometry: rd_geometry,
            properties: props
        }
    }
}

#[derive(Serialize, Deserialize, Clone)] 
pub struct InducedBevingenGeoJson {
    #[serde(rename = "type")]
    pub type_: String,
    pub features: Vec<InducedBevingGeoJson>

}

impl From<InducedBevingen> for InducedBevingenGeoJson {
    fn from(bevingen: InducedBevingen) -> Self {
        let mut features: Vec<InducedBevingGeoJson> = vec![];
        for v in bevingen.events {
            features.push(v.into())
        }
        InducedBevingenGeoJson { 
            type_: "FeatureCollection".to_owned(),
            features: features }
    }
}


pub async fn init_knmi_bevingen() -> Result<InducedBevingen, reqwest::Error> {

    //let mut induced_aardbevingen: Vec<InducedBevingData> = Vec::<InducedBevingData>::new();
    let resp = reqwest::get(r"https://cdn.knmi.nl/knmi/map/page/seismologie/all_induced.json")
        .await?;
    let data = resp.json::<InducedBevingen>()
        .await?;

    
    return Ok(data)
}
