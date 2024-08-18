use pyo3::prelude::*;
use serde_json::Value;

#[pyclass]
#[derive(Clone)]
pub struct Infobox {
    #[pyo3(get)]
    label: String,
    #[pyo3(get)]
    value: String,
}

#[pymethods]
impl Infobox {
    pub fn __repr__(&self) -> String {
        format!("Infobox(label={:?}, value={:?})", self.label, self.value)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Abstract {
    #[pyo3(get)]
    text: String,
    #[pyo3(get)]
    source: String,
    #[pyo3(get)]
    url: String,
    #[pyo3(get)]
    answer: String,
    #[pyo3(get)]
    definition: String,
    #[pyo3(get)]
    entity: String,
    #[pyo3(get)]
    heading: String,
    #[pyo3(get)]
    image: String,
    _infobox: Vec<Infobox>,
}

#[pymethods]
impl Abstract {
    pub fn __repr__(&self) -> String {
        format!(
            "Abstract(text={:?}, source={:?}, url={:?}, answer={:?}, definition={:?}, entity={:?}, heading={:?}, image={:?}, infobox=",
            self.text, 
            self.source, 
            self.url, 
            self.answer, 
            self.definition,
            self.entity,
            self.heading, 
            self.image
        ) 
        + "[" 
        + self._infobox.iter().map(|i|i.__repr__()).collect::<Vec<_>>().join(", ").as_str() 
        + "])"
    }
}

#[pyfunction]
pub fn get_abstract(embedded: String) -> PyResult<Abstract> {
    let embed = serde_json::from_str::<Value>(embedded.as_str());

    match embed {
        Ok(value) => {
            let data = &value["data"];
            let infoboxes = data["Infobox"]["content"].as_array();

            Ok(Abstract {
                text: data["AbstractText"].as_str().unwrap_or("").to_string(),
                source: data["AbstractSource"].as_str().unwrap_or("").to_string(),
                url: data["AbstractURL"].as_str().unwrap_or("").to_string(),
                answer: data["Answer"].as_str().unwrap_or("").to_string(),
                definition: data["Definition"].as_str().unwrap_or("").to_string(),
                entity: data["Entity"].as_str().unwrap_or("").to_string(),
                heading: data["Heading"].as_str().unwrap_or("").to_string(),
                image: data["Image"].as_str().unwrap_or("").to_string(),
                _infobox: infoboxes
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|infobox| Infobox {
                        label: infobox["label"].to_string(),
                        value: infobox["value"].to_string(),
                    })
                    .collect(),
            })
        }
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!("failed to get abstract embed: {}", e))),
    }
}
