use serde::{Serialize, Deserialize, Deserializer};
use geojson::Geometry;
use chrono::{DateTime, Utc};
use std::str::FromStr;
use std::fmt::Display;
use serde_aux::prelude::*;

pub type Url = String;


#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    geometry: Option<Geometry>,
    properties: Option<AssessmentProperties>,
    r#type: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssessmentProperties {
    #[serde(deserialize_with = "deserialize_option_bool_from_string")]
    air_conditioning: Option<bool>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    assessed_land_area: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    assessed_value_1: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    assessed_value_2: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    assessed_value_3: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    assessed_value_4: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    assessed_value_5: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_datetime_from_string")]
    assessment_date: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_option_bool_from_string")]
    attached_garage: Option<bool>,
    #[serde(deserialize_with = "deserialize_option_bool_from_string")]
    basement: Option<bool>,
    #[serde(deserialize_with = "deserialize_option_bool_from_string")]
    basement_finish: Option<bool>,
    building_type: Option<String>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    current_assessment_year: Option<usize>,
    #[serde(deserialize_with = "deserialize_option_bool_from_string")]
    detached_garage: Option<bool>,
    detail_url: Option<Url>,
    #[serde(deserialize_with = "deserialize_option_bool_from_string")]
    fire_place: Option<bool>,
    full_address: Option<String>,
    market_region: Option<String>,
    #[serde(deserialize_with = "deserialize_option_bool_from_string")]
    multiple_residences: Option<bool>,
    neighborhood_area: Option<String>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    number_floors_condo: Option<usize>,
    #[serde(deserialize_with = "deserialize_option_bool_from_string")]
    pool: Option<bool>,
    property_class_1: Option<String>,
    property_class_2: Option<String>,
    property_class_3: Option<String>,
    property_class_4: Option<String>,
    property_class_5: Option<String>,
    property_influences: Option<String>,
    property_use_code: Option<String>,
    #[serde(deserialize_with = "deserialize_option_datetime_from_string")]
    proposed_assessment_date: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    proposed_assessment_value_1: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    proposed_assessment_value_2: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    proposed_assessment_value_3: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    proposed_assessment_value_4: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    proposed_assessment_value_5: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    proposed_assessment_year: Option<usize>,
    proposed_property_class_1: Option<String>,
    proposed_property_class_2: Option<String>,
    proposed_property_class_3: Option<String>,
    proposed_property_class_4: Option<String>,
    proposed_property_class_5: Option<String>,
    proposed_status_1: Option<String>,
    proposed_status_2: Option<String>,
    proposed_status_3: Option<String>,
    proposed_status_4: Option<String>,
    proposed_status_5: Option<String>,
    roll_number: Option<String>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    rooms: Option<usize>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    sewer_frontage_measurement: Option<f64>,
    status_1: Option<String>,
    status_2: Option<String>,
    status_3: Option<String>,
    status_4: Option<String>,
    status_5: Option<String>,
    street_direction: Option<String>,
    street_name: Option<String>,
    street_number: Option<String>,
    street_suffix: Option<String>,
    street_type: Option<String>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    total_assessed_value: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    total_living_area: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    total_proposed_assessment_value: Option<f64>,
    unit_number: Option<String>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    water_frontage_measurement: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    year_built: Option<usize>,
    zoning: Option<String>
}


pub fn deserialize_option_datetime_from_string<'de, T, D>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum DateTimeOrNull<'a, T> {
        Str(&'a str),
        FromStr(T),
        Null,
    }

    match DateTimeOrNull::<T>::deserialize(deserializer)? {
        DateTimeOrNull::Str(s) => match s {
            "" => Ok(None),
            _ => T::from_str(&format!("{}Z", s)).map(Some).map_err(serde::de::Error::custom),
        },
        DateTimeOrNull::FromStr(i) => Ok(Some(i)),
        DateTimeOrNull::Null => Ok(None),
    }
}

pub fn deserialize_option_bool_from_string<'de, D>(
    deserializer: D,
) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BoolOrNull<'a> {
        Str(&'a str),
        FromStr(bool),
        Null,
    }

    match BoolOrNull::deserialize(deserializer)? {
        BoolOrNull::Str(s) => match s {
            "Yes" | "yes" | "true" => Ok(Some(true)),
            "No" | "no" | "false" => Ok(Some(false)),
            _ => bool::from_str(s).map(Some).map_err(serde::de::Error::custom),
        },
        BoolOrNull::FromStr(i) => Ok(Some(i)),
        BoolOrNull::Null => Ok(None),
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag="type")]
pub enum Features {
    FeatureCollection(FeatureCollection)
}

impl Default for Features {
    fn default() -> Self {
        Self::FeatureCollection(FeatureCollection::default())
    }
}

impl Features {
    pub fn len(&self) -> usize {
        let Self::FeatureCollection(collection) = self;
        collection.features.len()
    }
}


#[derive(Debug, Serialize, Deserialize, Default)]
#[serde()]
pub struct FeatureCollection {
    pub features: Vec<Record>
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_properties() {

        let s = r#"{"type": "FeatureCollection", "features": [{"geometry": null, "properties": null, "type": "Feature", "foo": "2017-04-07T11:11:23.348"}]}"#;
        assert!(serde_json::from_str::<Features>(s).is_ok());
        let f = serde_json::from_str::<Features>(s).unwrap();
        println!("{:#?}", f);
    }
}