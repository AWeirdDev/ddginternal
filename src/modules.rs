use pyo3::prelude::*;
use regex::Regex;

use crate::exceptions;
use crate::module_base::Module;
use crate::module_places;

#[pyfunction]
pub fn get_nrj_instances(djs: String) -> PyResult<Vec<(String, String)>> {
    let re = Regex::new(r"(?m)nrj\('(\/.+\.js.*)'.*,'(.+)'");

    if let Ok(re) = re {
        Ok(re
            .captures_iter(&djs)
            .map(|c| {
                (
                    c.get(1).unwrap().as_str().to_string(),
                    c.get(2).unwrap().as_str().to_string(),
                )
            })
            .collect::<Vec<_>>())
    } else {
        Err(exceptions::RegexError::new_err("failed to compile regex"))
    }
}

#[pyclass]
pub enum Assignee {
    Places(module_places::PlacesModule),
}

#[pymethods]
impl Assignee {
    fn who(&self) -> String {
        match self {
            Assignee::Places(_) => "places",
        }
        .to_string()
    }

    fn places(&self) -> PyResult<module_places::PlacesModule> {
        match self {
            Assignee::Places(m) => Ok(m.to_owned()),
            #[allow(unreachable_patterns)]
            _ => Err(pyo3::exceptions::PyNameError::new_err(
                "not a places module",
            )),
        }
    }
}

#[pyfunction]
pub fn assign_nrj_instances(instances: Vec<(String, String)>) -> Vec<Assignee> {
    let mut assignees = vec![];

    for (nrj, instance) in instances {
        match instance.as_str() {
            "places" => assignees.push(Assignee::Places(
                module_places::PlacesModule::from_instance(nrj),
            )),
            _ => println!(
                "ddginternal: warning: unimplemented nrj instance: {:?}",
                instance
            ),
        }
    }

    assignees
}
