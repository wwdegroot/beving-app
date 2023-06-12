use serde::{de, Serialize, Deserialize, Deserializer};
use serde_json::Value;
use reqwest;
use chrono::{NaiveDateTime};
use crate::{conversion::rijksdriehoekstelsel::WGS84Coordinate};

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InducedBevingen {
    pub events: Vec<InducedBeving>,
}

/// Geojson Point Geometry
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

/// Geojson Feature Properties
#[derive(Serialize, Deserialize, Clone)] 
pub struct InducedProperties {
    pub date: String,
    pub year: i32,
    pub depth: f64,
    #[serde(rename = "evaluationMode")]
    pub evaluation_mode: String,
    pub mag: f64,
    pub place: String,
    pub time: String,
    // datetime string als combinatie van date en time velden
    #[serde(with = "custom_date_format")]
    pub datetime: NaiveDateTime,
}

/// Serde datestring formatter
mod custom_date_format {
    use chrono::{NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};
    use crate::services::knmidata::parse_date_string;

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(parse_date_string(&s))
    }
}

/// Geojson Feature
#[derive(Serialize, Deserialize, Clone)] 
pub struct InducedBevingGeoJson {
    #[serde(rename = "type")]
    pub type_: String,
    pub geometry: PointRD,
    pub properties: InducedProperties,
    // datetime veld voor sorteren 
    #[serde(skip_serializing)]
    pub datetime: NaiveDateTime,

}

impl From<InducedBeving> for InducedBevingGeoJson {
    fn from(induced_beving: InducedBeving) -> Self {
        let wgs84: WGS84Coordinate = WGS84Coordinate::new(induced_beving.lat, induced_beving.lon);
        let rd_geometry: PointRD = wgs84.into();
        let datetime = parse_date_string(
            format!("{} {}", &induced_beving.date, &induced_beving.time).as_str()
        );
        let props = InducedProperties{
            date: induced_beving.date.clone(),
            year: get_year(&induced_beving.date),
            depth: induced_beving.depth,
            evaluation_mode: induced_beving.evaluation_mode,
            mag: induced_beving.mag,
            place: induced_beving.place,
            time: induced_beving.time.clone(),
            datetime: parse_date_string(format!("{} {}", induced_beving.date, induced_beving.time).as_str())
        };

        InducedBevingGeoJson{
            type_: "Feature".to_string(),
            geometry: rd_geometry,
            properties: props,
            datetime: datetime
        }
    }
}

/// Geojson FeatureCollection struct
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

/// Initieer de dataset vanuit knmi.
pub async fn init_knmi_bevingen() -> Result<InducedBevingen, reqwest::Error> {
    let resp = reqwest::get(r"https://cdn.knmi.nl/knmi/map/page/seismologie/all_induced.json")
        .await?;
    let data = resp.json::<InducedBevingen>()
        .await?;

    
    return Ok(data)
}

/// Functie die jaar uit date string haalt
fn get_year(datestring: &str) -> i32 {
    let parts: Vec<&str> = datestring.split('-').collect();
    let year = parts[0].parse::<i32>().unwrap();
    return year

}


/// Functie die date time string omzet in chrono NaiveDateTime
fn parse_date_string(datestring: &str) -> NaiveDateTime {
    let datetime = NaiveDateTime::parse_from_str(
        datestring, 
        "%Y-%m-%d %H:%M:%S"
    );
    match datetime {
        Ok(datetime) => datetime,
        Err(err) => {
            tracing::error!("Fout tijdens verwerken datetime: {}", err);
            NaiveDateTime::MIN
        }
    }   

}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn test_parse_date_string() {
        let datestring: &str = "2023-05-03 07:31:24".as_ref();
        let d = NaiveDate::from_ymd_opt(2023, 5, 3).unwrap();
        let t = NaiveTime::from_hms_opt(7,31,24).unwrap();
        let dt = NaiveDateTime::new(d, t);
        let parsed_datestring = parse_date_string(datestring);

        assert_eq!(parsed_datestring, dt)
    }
}