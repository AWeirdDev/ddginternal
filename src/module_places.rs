use std::collections::HashMap;

use pyo3::prelude::*;
use serde_json::Value;

use crate::module_base;

#[pyclass]
#[derive(Clone)]
pub struct PlacesModule {
    #[pyo3(get, name = "type")]
    type_: String,
    #[pyo3(get)]
    geoip_lat: f64,
    #[pyo3(get)]
    geoip_lon: f64,
    #[pyo3(get)]
    obfus_lat: f64,
    #[pyo3(get)]
    obfus_lon: f64,
    #[pyo3(get)]
    more_at: String,
    #[pyo3(get)]
    results: Vec<Place>,
}

#[pyclass]
#[derive(Clone)]
pub struct Place {
    #[pyo3(get)]
    address: String,
    #[pyo3(get)]
    address_lines: Vec<String>,
    #[pyo3(get)]
    city: String,
    #[pyo3(get)]
    closed: Option<u8>,
    #[pyo3(get)]
    lat: f64,
    #[pyo3(get)]
    lon: f64,
    #[pyo3(get)]
    country_code: Option<String>,
    #[pyo3(get)]
    category: String,
    #[pyo3(get)]
    display_phone: String,
    #[pyo3(get)]
    distance: f64,
    #[pyo3(get)]
    distance_to_user_meters: f64,
    #[pyo3(get)]
    facebook_id: String,
    #[pyo3(get)]
    hotel_id: Option<String>,
    #[pyo3(get)]
    hours: Hours,
    #[pyo3(get)]
    image: Option<String>,
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    photo: String,
    #[pyo3(get)]
    rating: u8,
    #[pyo3(get)]
    reviews: Vec<Review>,
}

#[pyclass]
#[derive(Clone)]
pub struct Hours {
    #[pyo3(get)]
    table: HashMap<String, String>,
    #[pyo3(get)]
    closes_soon: Option<u8>,
    #[pyo3(get)]
    opens_soon: Option<u8>,
    #[pyo3(get)]
    state_switch_time: Option<String>,
}

#[pyclass]
#[derive(Clone)]
pub struct Review {
    #[pyo3(get)]
    excerpt: String,
    #[pyo3(get)]
    rating: u8,
    #[pyo3(get)]
    time_created: u64,
    #[pyo3(get)]
    user: HashMap<String, String>,
}

impl module_base::Module for PlacesModule {
    fn from_instance(nrj: String) -> Self {
        let data = nrj.split_once("(").unwrap().1.strip_suffix(')').unwrap();
        let value = serde_json::from_str::<Value>(data).unwrap_or(Value::default());

        Self {
            type_: "places".to_string(),
            geoip_lat: value["geoip_lat"].as_f64().unwrap_or(0.0),
            geoip_lon: value["geoip_lon"].as_f64().unwrap_or(0.0),
            obfus_lat: value["obfus_lat"].as_f64().unwrap_or(0.0),
            obfus_lon: value["obfus_lon"].as_f64().unwrap_or(0.0),
            more_at: value["more_at"].as_str().unwrap_or("").to_string(),
            results: value["results"]
                .as_array()
                .unwrap_or(&Vec::new())
                .into_iter()
                .map(|ref place| Place {
                    address: place["address"].as_str().unwrap_or("").to_string(),
                    address_lines: place["address_lines"]
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .into_iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<_>>(),
                    city: place["city"].as_str().unwrap_or("").to_string(),
                    closed: place["closed"].as_u64().map(|c| c as u8),
                    lat: place["lat"].as_f64().unwrap_or(0.0),
                    lon: place["lon"].as_f64().unwrap_or(0.0),
                    country_code: place["country_code"].as_str().map(|i| i.to_string()),
                    category: place["category"].as_str().unwrap_or("").to_string(),
                    display_phone: place["display_phone"].as_str().unwrap_or("").to_string(),
                    distance: place["distance"].as_f64().unwrap_or(0.0),
                    distance_to_user_meters: place["distance_to_user_meters"]
                        .as_f64()
                        .unwrap_or(0.0),
                    facebook_id: place["facebook_id"].as_str().unwrap_or("").to_string(),
                    hotel_id: place["hotel_id"].as_str().map(|s| s.to_string()),
                    hours: Hours {
                        table: {
                            let mut table = HashMap::new();
                            place["hours"]["table"]
                                .as_object()
                                .unwrap_or(&serde_json::Map::new())
                                .iter()
                                .for_each(|(ref k, ref v)| {
                                    table.insert(
                                        k.to_string(),
                                        v.as_str().unwrap_or("").to_string(),
                                    );
                                });
                            table
                        },
                        closes_soon: place["hours"]["closes_soon"].as_u64().map(|i| i as u8),
                        opens_soon: place["hours"]["opens_soon"].as_u64().map(|i| i as u8),
                        state_switch_time: place["hours"]["state_switch_time"]
                            .as_str()
                            .map(|s| s.to_string()),
                    },
                    image: place["image"].as_str().map(|s| s.to_string()),
                    name: place["name"].as_str().unwrap_or("").to_string(),
                    photo: place["photo"].as_str().unwrap_or("").to_string(),
                    rating: place["rating"].as_u64().unwrap_or(0) as u8,
                    reviews: Vec::new(),
                })
                .collect::<Vec<_>>(),
        }
    }
}
