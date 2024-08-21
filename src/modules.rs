use pyo3::prelude::*;
use regex::Regex;

use crate::exceptions;
use crate::module_base::Module;
use crate::module_places;
use crate::module_recipes;

#[pyfunction]
pub fn get_nrj_instances(djs: String) -> PyResult<Vec<(String, String)>> {
    let re = Regex::new(r"(?m)nrj\('(/[^/]+\.js.*)'.*,'(.+?)'\);");
    let re2 = Regex::new(r"(?m)nrj\('(/js/spice/recipes/.+)',.+,'recipes'\);");

    if let Err(_) = re2 {
        return Err(exceptions::RegexError::new_err(
            "failed to compile regex (2)",
        ));
    }

    let re2 = re2.unwrap();
    if let Ok(re) = re {
        let mut results = re
            .captures_iter(&djs)
            .map(|c| {
                (
                    c.get(1).unwrap().as_str().to_string(),
                    c.get(2).unwrap().as_str().to_string(),
                )
            })
            .collect::<Vec<_>>();

        if let Some(m) = re2.captures_iter(&djs).next() {
            results.push((
                m.get(1).unwrap().as_str().to_string(),
                "recipes".to_string(),
            ));
        }

        Ok(results)
    } else {
        Err(exceptions::RegexError::new_err("failed to compile regex"))
    }
}

#[pyclass]
pub enum Assignee {
    Places(module_places::PlacesModule),
    Recipes(module_recipes::RecipesModule),
}

#[pymethods]
impl Assignee {
    fn who(&self) -> String {
        match self {
            Assignee::Places(_) => "places",
            Assignee::Recipes(_) => "recipes",
        }
        .to_string()
    }

    fn places(&self) -> PyResult<module_places::PlacesModule> {
        match self {
            Assignee::Places(m) => Ok(m.to_owned()),
            _ => Err(pyo3::exceptions::PyNameError::new_err(
                "not a places module",
            )),
        }
    }

    fn recipes(&self) -> PyResult<module_recipes::RecipesModule> {
        match self {
            Assignee::Recipes(m) => Ok(m.to_owned()),
            _ => Err(pyo3::exceptions::PyNameError::new_err(
                "not a recipes module",
            )),
        }
    }
}

#[pyfunction]
pub fn assign_nrj_instances(instances: Vec<(String, String)>) -> Vec<Assignee> {
    let mut assignees = vec![];

    for (nrj, instance) in instances {
        match instance.as_str() {
            "maps_places" => {
                let binding = module_places::PlacesModule::from_instance(nrj);
                if let Some(p) = binding {
                    assignees.push(Assignee::Places(p))
                }
            }
            "recipes" => {
                let binding = module_recipes::RecipesModule::from_instance(nrj);
                if let Some(r) = binding {
                    assignees.push(Assignee::Recipes(r))
                }
            }
            _ => println!(
                "ddginternal: warning: unimplemented nrj instance: {:?}",
                instance
            ),
        }
    }

    assignees
}
