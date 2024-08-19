use std::collections::HashMap;

use pyo3::prelude::*;
use serde_json::Value;

use crate::module_base::Module;

#[pyclass]
#[derive(Clone)]
pub struct RecipesModule {
    #[pyo3(get, name = "type")]
    type_: String,
    #[pyo3(get)]
    results: Vec<Recipe>,
}

#[pyclass]
#[derive(Clone)]
pub struct Recipe {
    #[pyo3(get)]
    vegetarian: bool,
    #[pyo3(get)]
    vegan: bool,
    #[pyo3(get)]
    gluten_free: bool,
    #[pyo3(get)]
    dairy_free: bool,
    #[pyo3(get)]
    very_healthy: bool,
    #[pyo3(get)]
    cheap: bool,
    #[pyo3(get)]
    very_popular: bool,
    #[pyo3(get)]
    sustainable: bool,
    #[pyo3(get)]
    low_fodmap: bool,
    #[pyo3(get)]
    weight_watcher_smart_points: u32,
    #[pyo3(get)]
    gaps: String,
    #[pyo3(get)]
    preparation_minutes: u32,
    #[pyo3(get)]
    cooking_minutes: u16,
    #[pyo3(get)]
    ready_in_minutes: u32,
    #[pyo3(get)]
    aggregate_likes: u32,
    #[pyo3(get)]
    health_score: u32,
    #[pyo3(get)]
    credits_text: String,
    #[pyo3(get)]
    source_name: String,
    #[pyo3(get)]
    price_per_serving: f64,
    #[pyo3(get)]
    extended_ingredients: Vec<ExtendedIngredient>,
    #[pyo3(get)]
    title: String,
    #[pyo3(get)]
    servings: u32,
    #[pyo3(get)]
    source_url: String,
    #[pyo3(get)]
    image: String,
    #[pyo3(get)]
    raw_summary: String,
    #[pyo3(get)]
    cuisines: Vec<String>,
    #[pyo3(get)]
    dish_types: Vec<String>,
    #[pyo3(get)]
    diets: Vec<String>,
    #[pyo3(get)]
    occasions: Vec<String>,
    #[pyo3(get)]
    spoonacular_score: f64,
}

#[pymethods]
impl Recipe {
    #[getter]
    fn summary(&self) -> String {
        html2text::from_read(self.raw_summary.as_bytes(), usize::MAX)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ExtendedIngredient {
    #[pyo3(get)]
    aisle: String,
    #[pyo3(get)]
    consistency: String,
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    name_clean: String,
    #[pyo3(get)]
    original: String,
    #[pyo3(get)]
    original_name: String,
    #[pyo3(get)]
    amount: f64,
    #[pyo3(get)]
    unit: String,
    #[pyo3(get)]
    meta: Vec<String>,
    #[pyo3(get)]
    measures: HashMap<String, Measure>,
}

#[pyclass]
#[derive(Clone)]
pub struct Measure {
    #[pyo3(get)]
    amount: f64,
    #[pyo3(get)]
    unit_short: String,
    #[pyo3(get)]
    unit_long: String,
}

impl Module for RecipesModule {
    fn from_instance(nrj: String) -> Self {
        let data = nrj.split_once('(').unwrap().1.strip_suffix(");").unwrap();
        let value = serde_json::from_str::<Value>(data).unwrap_or(Value::default());

        Self {
            type_: "recipes".to_string(),
            results: value["results"]
                .as_array()
                .unwrap()
                .iter()
                .map(|result| Recipe {
                    vegetarian: result["vegetarian"].as_bool().unwrap_or(false),
                    vegan: result["vegan"].as_bool().unwrap_or(false),
                    gluten_free: result["glutenFree"].as_bool().unwrap_or(false),
                    dairy_free: result["dairyFree"].as_bool().unwrap_or(false),
                    very_healthy: result["veryHealthy"].as_bool().unwrap_or(false),
                    cheap: result["cheap"].as_bool().unwrap_or(false),
                    very_popular: result["veryPopular"].as_bool().unwrap_or(false),
                    sustainable: result["sustainable"].as_bool().unwrap_or(false),
                    low_fodmap: result["lowFodmap"].as_bool().unwrap_or(false),
                    weight_watcher_smart_points: result["weightWatcherSmartPoints"]
                        .as_u64()
                        .unwrap_or(0) as u32,
                    gaps: result["gaps"].as_str().unwrap_or("").to_string(),
                    preparation_minutes: result["preparationMinutes"].as_u64().unwrap_or(0) as u32,
                    cooking_minutes: result["cookingMinutes"].as_u64().unwrap_or(0) as u16,
                    ready_in_minutes: result["readyInMinutes"].as_u64().unwrap_or(0) as u32,
                    aggregate_likes: result["aggregateLikes"].as_u64().unwrap_or(0) as u32,
                    health_score: result["healthScore"].as_u64().unwrap_or(0) as u32,
                    credits_text: result["creditsText"].as_str().unwrap_or("").to_string(),
                    source_name: result["sourceName"].as_str().unwrap_or("").to_string(),
                    price_per_serving: result["pricePerServing"].as_f64().unwrap_or(0.0),
                    servings: result["servings"].as_u64().unwrap_or(0) as u32,
                    source_url: result["sourceUrl"].as_str().unwrap_or("").to_string(),
                    image: result["image"].as_str().unwrap_or("").to_string(),
                    raw_summary: result["summary"].as_str().unwrap_or("").to_string(),
                    cuisines: result["cuisines"]
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect(),
                    dish_types: result["dishTypes"]
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect(),
                    diets: result["diets"]
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect(),
                    occasions: result["occasions"]
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect(),
                    spoonacular_score: result["spoonacularScore"].as_f64().unwrap_or(0.0),
                    title: result["title"].as_str().unwrap_or("").to_string(),
                    extended_ingredients: result["ingredients"]
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .map(|ing| ExtendedIngredient {
                            aisle: ing["aisle"].as_str().unwrap_or("").to_string(),
                            consistency: ing["consistency"].as_str().unwrap_or("").to_string(),
                            name: ing["name"].as_str().unwrap_or("").to_string(),
                            name_clean: ing["nameClean"].as_str().unwrap_or("").to_string(),
                            original: ing["original"].as_str().unwrap_or("").to_string(),
                            original_name: ing["originalName"].as_str().unwrap_or("").to_string(),
                            amount: ing["amount"].as_f64().unwrap_or(0.0),
                            unit: ing["unit"].as_str().unwrap_or("").to_string(),
                            meta: ing["meta"]
                                .as_array()
                                .unwrap_or(&Vec::new())
                                .iter()
                                .map(|v| v.as_str().unwrap_or("").to_string())
                                .collect(),
                            measures: {
                                let mut table = HashMap::new();
                                ing["measures"]
                                    .as_object()
                                    .unwrap_or(&serde_json::Map::new())
                                    .iter()
                                    .for_each(|(ref k, ref v)| {
                                        let vv = v.as_object().unwrap();
                                        table.insert(
                                            k.to_string(),
                                            Measure {
                                                amount: vv["amount"].as_f64().unwrap_or(0.0),
                                                unit_short: vv["unitShort"]
                                                    .as_str()
                                                    .unwrap_or("")
                                                    .to_string(),
                                                unit_long: vv["unitLong"]
                                                    .as_str()
                                                    .unwrap_or("")
                                                    .to_string(),
                                            },
                                        );
                                    });
                                table
                            },
                        })
                        .collect(),
                })
                .collect::<Vec<_>>(),
        }
    }
}
